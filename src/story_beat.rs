use itertools::Itertools;
use std::{collections::HashMap, fmt::Display, ops::Range, rc::Rc};

pub const ID: &str = "id";

pub type Alias = String;
pub type PropertyName = String;
pub type PropertyMap = HashMap<PropertyName, Property>;
// key is a pair of ids, value is property from POV of 1st entity
pub type RelationMap = HashMap<(Property, Property), PropertyMap>;

pub struct Query {
    pub entities: Vec<PropertyMap>, // characters, items, locations ... matched against alias_constraints
    pub entity_relations: RelationMap,
    pub world_state: PropertyMap, // miscellanious world variables, matched agains world_constraints
                                  //* discard: Vec<StoryBeat>, // TODO: some kind of identifier? uuid?
}

pub struct ConstrainedAlias(pub Alias, pub Vec<PropertyConstraint>);

#[derive(Default)]
pub struct StoryBeat {
    pub description: String,
    pub aliases: Vec<ConstrainedAlias>,
    pub relation_constraints: Vec<RelationConstraint>,
    pub world_constraints: Vec<WorldConstraint>,
    pub directives: Vec<String>, // TODO, some DSL instead of just strings? maybe this approach https://github.com/clap-rs/clap/blob/053c778e986d99b4f53afdb666d9398e75d8d2fb/examples/repl.rs
    children: Vec<Rc<StoryBeat>>,
}

impl StoryBeat {
    pub fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}

pub struct StoryBeatBuilder(StoryBeat);

impl StoryBeatBuilder {
    pub fn new() -> Self {
        Self(StoryBeat::default())
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.0.description = description.into();
        self
    }

    pub fn with_alias(
        mut self,
        alias: impl Into<String>,
        constraints: Vec<PropertyConstraint>,
    ) -> Self {
        self.0
            .aliases
            .push(ConstrainedAlias(alias.into(), constraints));
        self
    }

    pub fn with_relation(
        mut self,
        me: impl Into<Alias>,
        other: impl Into<Alias>,
        constraint: PropertyConstraint,
    ) -> Self {
        self.0.relation_constraints.push(RelationConstraint {
            me: me.into(),
            other: other.into(),
            constraint,
        });
        self
    }

    pub fn build(self) -> StoryBeat {
        self.0
    }
}

impl Default for StoryBeatBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub enum PropertyConstraint {
    Has(PropertyName),
    IsInRange(PropertyName, Range<i64>),
}

impl PropertyConstraint {
    pub fn is_satisfied_by(&self, entity: &PropertyMap) -> bool {
        match self {
            PropertyConstraint::Has(prop_name) => entity.contains_key(prop_name),
            PropertyConstraint::IsInRange(prop_name, range) => {
                entity.get(prop_name).is_some_and(|prop| match prop {
                    Property::Int(value) => range.contains(value),
                    _ => false,
                })
            }
        }
    }
}

pub struct RelationConstraint {
    pub me: Alias,
    pub other: Alias,
    pub constraint: PropertyConstraint,
}

pub enum WorldConstraint {
    HasProperty(PropertyName),
}

// TODO: constraints that check at least, between, etc. fail on string prop
#[derive(Hash, PartialEq, Eq, Clone)]
pub enum Property {
    String(String),
    Int(i64),
}

impl Display for Property {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Property::String(s) => write!(fmt, "{}", s),
            Property::Int(i) => write!(fmt, "{}", i),
            // PropType::Float(f) => write!(fmt, "{}", f),
        }
    }
}

impl From<String> for Property {
    fn from(val: String) -> Self {
        Property::String(val)
    }
}

impl From<i64> for Property {
    fn from(val: i64) -> Self {
        Property::Int(val)
    }
}

// impl From<f64> for PropType {
//     fn from(val: f64) -> Self {
//         PropType::Float(val)
//     }
// }

pub struct Raconteur {
    story_beats: Vec<Rc<StoryBeat>>,
}

impl Raconteur {
    pub fn new() -> Self {
        Self {
            story_beats: vec![],
        }
    }

    pub fn push(&mut self, story_beat: StoryBeat) {
        self.story_beats.push(Rc::new(story_beat))
    }

    // Returns a pair of valid story beat with its list of valid aliased entities
    // inner vec is a list of permutations of indices. first index is for first alias, etc.
    pub fn query(&self, query: &Query) -> Vec<(Rc<StoryBeat>, Vec<Vec<usize>>)> {
        // go through list of story beats, discarding those whose constraints aren't satisfied

        self.story_beats
            .iter()
            .filter_map(|beat| {
                let alias_candidates = are_all_constraints_satisfied(beat, query);
                if alias_candidates.is_empty() {
                    None
                } else {
                    Some((Rc::clone(beat), alias_candidates))
                }
            })
            .collect_vec()
    }
}

impl Default for Raconteur {
    fn default() -> Self {
        Self::new()
    }
}

fn are_all_constraints_satisfied(beat: &Rc<StoryBeat>, query: &Query) -> Vec<Vec<usize>> {
    if !are_world_constraints_satisfied(beat, query) {
        return vec![];
    }
    match_aliases(beat, query)
}

fn are_world_constraints_satisfied(beat: &Rc<StoryBeat>, query: &Query) -> bool {
    beat.world_constraints
        .iter()
        .all(|constraint| match constraint {
            WorldConstraint::HasProperty(prop_name) => query.world_state.contains_key(prop_name),
        })
}

fn match_aliases(beat: &Rc<StoryBeat>, query: &Query) -> Vec<Vec<usize>> {
    if query.entities.len() < beat.aliases.len() {
        return vec![];
    }

    // get all valid entity indices for each alias
    let alias_candidate_indices = beat
        .aliases
        .iter()
        .map(|constrained_alias| {
            let ConstrainedAlias(alias, constraints) = constrained_alias;

            // produce list of valid entity indices
            let valid_indices = query
                .entities
                .iter()
                .enumerate()
                .filter_map(|(index, entity)| {
                    constraints
                        .iter()
                        .all(|constraint| constraint.is_satisfied_by(entity))
                        .then_some(index)
                })
                .collect_vec();
            (alias, valid_indices)
        })
        .collect_vec();

    // produce all unique permutation of character indices for each alias
    // To use itertools' cartesian product, must first populate the permutations vector once
    let mut alias_permutations = Vec::<Vec<usize>>::default();
    alias_candidate_indices[0]
        .1
        .iter()
        .for_each(|index| alias_permutations.push(vec![*index]));
    for alias_candidates in alias_candidate_indices.iter().skip(1) {
        let (_, candidate_indices) = alias_candidates;
        alias_permutations = alias_permutations
            .into_iter()
            .cartesian_product(candidate_indices.iter().cloned())
            .filter_map(|(mut indices, new_index)| {
                let is_unique = !indices.contains(&new_index);
                is_unique.then(|| {
                    indices.push(new_index);
                    indices
                })
            })
            .collect();
    }
    alias_permutations.retain(|permutation| permutation.len() == beat.aliases.len());

    alias_permutations
        .into_iter()
        .filter(|entity_indices| {
            // long winded approach to getting ids
            beat.relation_constraints.iter().all(|relation| {
                let me_alias_idx = beat
                    .aliases
                    .iter()
                    .enumerate()
                    .find(|(_, constrained_alias)| {
                        let ConstrainedAlias(alias, _) = constrained_alias;
                        alias == &relation.me
                    })
                    .map(|(idx, _)| idx)
                    .unwrap();
                let other_alias_idx = beat
                    .aliases
                    .iter()
                    .enumerate()
                    .find(|(_, constrained_alias)| {
                        let ConstrainedAlias(alias, _) = constrained_alias;
                        alias == &relation.other
                    })
                    .map(|(idx, _)| idx)
                    .unwrap();
                let me_idx = entity_indices[me_alias_idx];
                let other_idx = entity_indices[other_alias_idx];
                let me_id = query.entities[me_idx].get(ID).unwrap().clone();
                let other_id = query.entities[other_idx].get(ID).unwrap().clone();

                query
                    .entity_relations
                    .get(&(me_id, other_id))
                    .is_some_and(|prop_map| match &relation.constraint {
                        PropertyConstraint::Has(prop_name) => prop_map.contains_key(prop_name),
                        PropertyConstraint::IsInRange(prop_name, range) => {
                            prop_map.get(prop_name).is_some_and(|prop| match prop {
                                Property::Int(value) => range.contains(value),
                                _ => false,
                            })
                        }
                    })
            })
        })
        .collect()
}
