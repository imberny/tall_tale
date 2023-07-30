use std::{collections::HashMap, error::Error, fmt};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    constraint::{AliasRelation, Constraint},
    entity::EntityId,
    property::{PropertyMap, PropertyName},
    story_world::StoryWorld,
};

pub type Alias = String;
pub type AliasCandidates = HashMap<Alias, usize>;

#[derive(Debug)]
pub struct NotSatisfied;
impl fmt::Display for NotSatisfied {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Story node constraints not satisfied")
    }
}
impl Error for NotSatisfied {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstrainedAlias {
    alias: Alias,
    constraints: Vec<Constraint>,
}

impl ConstrainedAlias {
    pub fn new<A, C>(alias: A, constraints: C) -> Self
    where
        A: Into<Alias>,
        C: IntoIterator<Item = Constraint>,
    {
        Self {
            alias: alias.into(),
            constraints: Vec::from_iter(constraints),
        }
    }

    pub fn alias(&self) -> &Alias {
        &self.alias
    }

    pub fn is_satisfied_by(&self, properties: &PropertyMap) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_satisfied_by(properties))
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct StoryNode {
    pub description: String,
    pub aliases: Vec<ConstrainedAlias>,
    pub relation_constraints: Vec<AliasRelation>,
    pub world_constraints: Vec<Constraint>,
    pub directives: Vec<String>, // TODO, some DSL instead of just strings? maybe this approach https://github.com/clap-rs/clap/blob/053c778e986d99b4f53afdb666d9398e75d8d2fb/examples/repl.rs
}

impl StoryNode {
    pub fn new() -> Self {
        Self::default()
    }

    // builder methods

    pub fn with_description<S>(mut self, description: S) -> Self
    where
        S: Into<PropertyName>,
    {
        self.description = description.into();
        self
    }

    pub fn with_alias_constraints<A, C>(mut self, alias: A, constraints: C) -> Self
    where
        A: Into<Alias>,
        C: IntoIterator<Item = Constraint>,
    {
        self.aliases.push(ConstrainedAlias::new(alias, constraints));
        self
    }

    pub fn with_relation_constraints<A, C>(mut self, me: A, other: A, constraints: C) -> Self
    where
        A: Into<Alias>,
        C: IntoIterator<Item = Constraint>,
    {
        self.relation_constraints
            .push(AliasRelation::new(me, other, constraints));
        self
    }

    pub fn with_world_constraint(mut self, constraint: Constraint) -> Self {
        self.world_constraints.push(constraint);
        self
    }

    pub fn with_directive<D>(mut self, directive: D) -> Self
    where
        D: Into<String>,
    {
        self.directives.push(directive.into());
        self
    }

    pub(crate) fn try_matching_aliases(
        &self,
        query: &StoryWorld,
    ) -> Result<Vec<AliasCandidates>, NotSatisfied> {
        if !self.are_world_constraints_satisfied(query) {
            return Err(NotSatisfied);
        }

        self.find_alias_candidates(query)
    }

    pub fn are_world_constraints_satisfied(&self, query: &StoryWorld) -> bool {
        self.world_constraints
            .iter()
            .all(|constraint| constraint.is_satisfied_by(&query.world_properties))
    }

    // Returns list of permutations of entity ids
    // TODO: how to improve this? this is unreadable
    pub fn find_alias_candidates(
        &self,
        query: &StoryWorld,
    ) -> Result<Vec<AliasCandidates>, NotSatisfied> {
        if query.entities.len() < self.aliases.len() {
            return Err(NotSatisfied);
        }

        // get all valid entity indices for each alias
        // first entry is for first alias in list, second for second alias etc.
        let alias_candidate_indices = self
            .aliases
            .iter()
            .map(|constrained_alias| {
                // produce list of valid entity indices
                let valid_indices = query
                    .entities
                    .iter()
                    .filter_map(|entity| {
                        constrained_alias
                            .is_satisfied_by(&entity.properties)
                            .then_some(entity.id().0)
                    })
                    .collect_vec();
                valid_indices
            })
            .collect_vec();

        // produce all unique permutation of character indices for each alias
        // To use itertools' cartesian product, must first populate the permutations vector once
        // PERF: replace inner vec by Smallvec (which size? 5, 8, 20?)
        let mut alias_permutations = Vec::<Vec<usize>>::default();
        alias_candidate_indices[0]
            .iter()
            .for_each(|id| alias_permutations.push(vec![*id]));
        for alias_candidates in alias_candidate_indices.iter().skip(1) {
            let candidate_ids = alias_candidates;
            alias_permutations = alias_permutations
                .into_iter()
                .cartesian_product(candidate_ids.iter().cloned())
                .filter_map(|(mut ids, new_id)| {
                    let is_unique = !ids.contains(&new_id);
                    is_unique.then(|| {
                        ids.push(new_id);
                        ids
                    })
                })
                .collect();
        }
        alias_permutations.retain(|permutation| permutation.len() == self.aliases.len());

        // long winded approach to getting ids
        let get_id = |target_alias: &Alias, entity_ids: &Vec<usize>| {
            let alias_index = self
                .aliases
                .iter()
                .enumerate()
                .find(|(_, constrained_alias)| constrained_alias.alias() == target_alias)
                .map(|(idx, _)| idx)
                .unwrap();
            entity_ids[alias_index]
        };

        let valid_permutations = alias_permutations
            .into_iter()
            .filter(|permutation_ids| {
                self.relation_constraints.iter().all(|relation| {
                    let me_id = EntityId(get_id(&relation.me, permutation_ids));
                    let other_id = EntityId(get_id(&relation.other, permutation_ids));

                    query
                        .entity_relations
                        .get(&(me_id, other_id))
                        .is_some_and(|properties| relation.is_satisfied_by(properties))
                })
            })
            .map(|permutation| {
                permutation
                    .into_iter()
                    .enumerate()
                    .map(|(alias_index, entity_id)| {
                        let alias = self.aliases[alias_index].alias.clone();
                        (alias, entity_id)
                    })
                    .collect()
            })
            .collect_vec();

        if valid_permutations.is_empty() {
            Err(NotSatisfied)
        } else {
            Ok(valid_permutations)
        }
    }
}
