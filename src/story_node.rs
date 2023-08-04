use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    constraint::{AliasRelation, Constraint},
    property::{PropertyMap, PropertyName},
    story_graph::AliasMap,
    story_world::StoryWorld,
};

pub type Alias = String;

#[derive(Debug)]
pub struct NotSatisfied;
impl fmt::Display for NotSatisfied {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Story node constraints not satisfied")
    }
}
impl Error for NotSatisfied {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct ConstrainedAlias {
    alias: Alias,
    constraints: Vec<Constraint>,
}

impl ConstrainedAlias {
    pub(crate) fn new<A, C>(alias: A, constraints: C) -> Self
    where
        A: Into<Alias>,
        C: IntoIterator<Item = Constraint>,
    {
        Self {
            alias: alias.into(),
            constraints: Vec::from_iter(constraints),
        }
    }

    pub(crate) fn alias(&self) -> &Alias {
        &self.alias
    }

    pub(crate) fn is_satisfied_by(&self, properties: &PropertyMap) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.is_satisfied_by(properties))
    }
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryNode {
    pub description: String,
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

    pub(crate) fn are_world_constraints_satisfied(&self, story_world: &StoryWorld) -> bool {
        self.world_constraints
            .iter()
            .all(|constraint| constraint.is_satisfied_by(&story_world.properties))
    }

    pub(crate) fn are_relation_constraints_satisfied(
        &self,
        story_world: &StoryWorld,
        alias_entities: &AliasMap,
    ) -> bool {
        self.relation_constraints.iter().all(|relation| {
            let me_id = if let Some(id) = alias_entities.get(&relation.me) {
                id
            } else {
                return false;
            };
            let other_id = if let Some(id) = alias_entities.get(&relation.other) {
                id
            } else {
                return false;
            };

            let default_props = PropertyMap::default();
            let relation_properties = story_world
                .relations
                .get(&(me_id, other_id))
                .unwrap_or(&default_props);
            relation.is_satisfied_by(relation_properties)
        })
    }
}
