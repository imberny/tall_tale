use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::{
    constraint::{AliasRelation, Constraint},
    property::{PropertyMap, PropertyName},
    query::Query,
    raconteur::EntityIndex,
};

pub type Alias = String;

#[derive(Default, Serialize, Deserialize)]
struct ConstrainedAlias {
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

#[derive(Default, Serialize, Deserialize)]
pub struct StoryNode {
    pub description: String,
    aliases: Vec<ConstrainedAlias>,
    relation_constraints: Vec<AliasRelation>,
    world_constraints: Vec<Constraint>,
    pub directives: Vec<String>, // TODO, some DSL instead of just strings? maybe this approach https://github.com/clap-rs/clap/blob/053c778e986d99b4f53afdb666d9398e75d8d2fb/examples/repl.rs
}

impl StoryNode {
    pub fn new() -> Self {
        Self::default()
    }

    // builder methods
    //

    pub fn with_description<S>(mut self, description: S) -> Self
    where
        S: Into<String>,
    {
        self.description = description.into();
        self
    }

    pub fn with_alias<A, C>(mut self, alias: A, constraints: C) -> Self
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

    pub fn with_world_constraint<N>(mut self, property_name: N) -> Self
    where
        N: Into<PropertyName>,
    {
        self.world_constraints
            .push(Constraint::Has(property_name.into()));
        self
    }

    pub fn are_world_constraints_satisfied(&self, query: &Query) -> bool {
        self.world_constraints
            .iter()
            .all(|constraint| constraint.is_satisfied_by(&query.world_state))
    }

    pub fn find_alias_candidates(&self, query: &Query) -> Vec<Vec<usize>> {
        if query.entities.len() < self.aliases.len() {
            return vec![];
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
                    .enumerate()
                    .filter_map(|(entity_index, entity)| {
                        constrained_alias
                            .is_satisfied_by(&entity.properties)
                            .then_some(entity_index)
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
            .for_each(|index| alias_permutations.push(vec![*index]));
        for alias_candidates in alias_candidate_indices.iter().skip(1) {
            let candidate_indices = alias_candidates;
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
                .find(|(_, constrained_alias)| constrained_alias.alias() == target_alias)
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
                        .is_some_and(|properties| relation.is_satisfied_by(properties))
                })
            })
            .collect()
    }
}
