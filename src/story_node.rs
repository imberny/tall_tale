use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    constraint::{PropertyConstraint, RelationConstraint},
    property::Property,
    query::Query,
    raconteur::EntityIndex,
};

pub type Alias = String;
pub type AliasIndex = usize;

#[derive(Clone, Serialize, Deserialize)]
pub struct ConstrainedAlias(pub Alias, pub Vec<PropertyConstraint>);

#[derive(Default, Serialize, Deserialize)]
pub struct StoryNode {
    pub description: String,
    pub aliases: Vec<ConstrainedAlias>,
    pub relation_constraints: Vec<RelationConstraint>,
    pub world_constraints: Vec<PropertyConstraint>,
    pub directives: Vec<String>, // TODO, some DSL instead of just strings? maybe this approach https://github.com/clap-rs/clap/blob/053c778e986d99b4f53afdb666d9398e75d8d2fb/examples/repl.rs
}

impl StoryNode {
    pub fn new() -> Self {
        Self::default()
    }

    // builder methods

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_alias<T: Into<Alias>>(
        mut self,
        alias: T,
        constraints: Vec<PropertyConstraint>,
    ) -> Self {
        self.aliases
            .push(ConstrainedAlias(alias.into(), constraints));
        self
    }

    pub fn with_relation<T: Into<Alias>>(
        mut self,
        me: T,
        other: T,
        constraint: PropertyConstraint,
    ) -> Self {
        self.relation_constraints.push(RelationConstraint {
            me: me.into(),
            other: other.into(),
            constraint,
        });
        self
    }

    pub fn are_world_constraints_satisfied(&self, query: &Query) -> bool {
        self.world_constraints
            .iter()
            .all(|constraint| match constraint {
                PropertyConstraint::Has(prop_name) => query.world_state.contains_key(prop_name),
                PropertyConstraint::IsInRange(prop_name, range) => query
                    .world_state
                    .get(prop_name)
                    .is_some_and(|prop| match prop {
                        Property::Int(value) => range.contains(value),
                        _ => false,
                    }),
            })
    }

    pub fn find_alias_candidates(&self, query: &Query) -> Vec<Vec<usize>> {
        if query.entities.len() < self.aliases.len() {
            return vec![];
        }

        // get all valid entity indices for each alias
        let alias_candidate_indices = self
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
        alias_permutations.retain(|permutation| permutation.len() == self.aliases.len());

        // long winded approach to getting ids
        let get_id = |target_alias: &Alias, entity_indices: &Vec<EntityIndex>| {
            let alias_index = self
                .aliases
                .iter()
                .enumerate()
                .find(|(_, constrained_alias)| {
                    let ConstrainedAlias(alias, _) = constrained_alias;
                    alias == target_alias
                })
                .map(|(idx, _)| idx)
                .unwrap();
            let entity_index = entity_indices[alias_index];
            query.entities[entity_index].id()
        };

        alias_permutations
            .into_iter()
            .filter(|entity_indices| {
                self.relation_constraints.iter().all(|relation| {
                    let me_id = get_id(&relation.me, entity_indices);
                    let other_id = get_id(&relation.other, entity_indices);

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
}
