use std::{
    error::Error,
    fmt::{self, Write},
};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::{
    constraint::{AliasRelation, Constraint},
    context::Context,
    prelude::Entity,
    property::{PropertyMap, PropertyName},
    story_graph::{AliasError, AliasMap},
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

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
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

    //
    pub(crate) fn is_satisfied_by(&self, entity: &Entity) -> bool {
        entity
            .exclusory_properties
            .iter()
            .all(|(exclusory_prop_name, exclusory_property)| {
                // TODO: make constraints a hashmap of prop_name to constraint
                self.constraints.iter().any(|constraint| match constraint {
                    Constraint::Has(prop_name) => prop_name == exclusory_prop_name,
                    Constraint::Equals(prop_name, property) => {
                        prop_name == exclusory_prop_name && property == exclusory_property
                    }
                    Constraint::IsInRange(prop_name, range) => {
                        prop_name == exclusory_prop_name && exclusory_property.is_in_range(range)
                    }
                    Constraint::IsInRangeFloat(prop_name, range) => {
                        prop_name == exclusory_prop_name
                            && exclusory_property.is_in_range_float(range)
                    }
                    _ => false,
                })
            })
            && self.constraints.iter().all(|constraint| {
                constraint.is_satisfied_by(&entity.properties)
                    || constraint.is_satisfied_by(&entity.exclusory_properties)
            })
    }
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
pub struct StoryNode {
    pub description: String,
    pub relation_constraints: Vec<AliasRelation>,
    pub world_constraints: Vec<Constraint>,
    pub directive: String, // TODO, some DSL instead of just strings? maybe this approach https://github.com/clap-rs/clap/blob/053c778e986d99b4f53afdb666d9398e75d8d2fb/examples/repl.rs
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
        self.directive = directive.into();
        self
    }

    pub(crate) fn are_world_constraints_satisfied(&self, context: &Context) -> bool {
        self.world_constraints
            .iter()
            .all(|constraint| constraint.is_satisfied_by(context.properties()))
    }

    pub(crate) fn are_relation_constraints_satisfied(
        &self,
        context: &Context,
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
            let relation_properties = context
                .relations()
                .get(&(me_id, other_id))
                .unwrap_or(&default_props);
            relation.is_satisfied_by(relation_properties)
        })
    }

    pub fn directive(&self, alias_map: &AliasMap, context: &Context) -> Result<String, AliasError> {
        static PATTERNS_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"\{([<>[:word:]\.]+)\}").unwrap());
        static ALIAS_RE: Lazy<Regex> = Lazy::new(|| Regex::new("<a>([[:word:]]+)").unwrap());
        static ALIAS_PROP_RE: Lazy<Regex> =
            Lazy::new(|| Regex::new("([[:word:]]+)\\.(?P<property_name>[[:word:]]+)").unwrap());
        static WORLD_PROP_RE: Lazy<Regex> = Lazy::new(|| Regex::new("([[:word:]]+)").unwrap());

        let mut result_directive = String::new();

        let parts: Vec<&str> = PATTERNS_RE.split(&self.directive).collect_vec();
        let matches = PATTERNS_RE.captures_iter(&self.directive).collect_vec();
        for i in 0..matches.len() {
            write!(&mut result_directive, "{}", parts[i]).unwrap();
            let pattern = &matches[i][1];

            let unaliased_text: Result<_, _>;
            if let Some(cap) = ALIAS_RE.captures(pattern) {
                let alias = &cap[1];
                unaliased_text = alias_map
                    .get(alias)
                    .ok_or(AliasError::new(format!(r#"missing alias "{}""#, alias)))
                    .map(|entity_id: usize| entity_id.to_string());
            } else if let Some(cap) = ALIAS_PROP_RE.captures(pattern) {
                let alias = &cap[1];
                let property_name = &cap[2];
                unaliased_text = alias_map
                    .get(alias)
                    .ok_or(AliasError::new(format!(r#"missing alias "{}""#, alias)))
                    .and_then(|entity_id| {
                        context
                            .entity(entity_id)
                            .ok_or(AliasError::new(format!(
                                r#"Entity "{}" bound to "{}" is missing"#,
                                entity_id, alias
                            )))
                            .and_then(|entity| {
                                entity
                                    .get(property_name)
                                    .ok_or(AliasError::new(format!(
                                        r#"Entity "{}" bound to "{}" is missing the property "{}""#,
                                        entity_id, alias, property_name
                                    )))
                                    .map(|property| property.to_string())
                            })
                    });
            } else if let Some(cap) = WORLD_PROP_RE.captures(pattern) {
                let property_name = &cap[1];
                unaliased_text = context
                    .world_property(property_name)
                    .ok_or(AliasError::new(format!(
                        r#"Missing world property "{}""#,
                        property_name
                    )))
                    .map(|property| property.to_string());
            } else {
                return Err(AliasError::new(format!(
                    r#"Malformed property path "{}""#,
                    pattern
                )));
            }

            match unaliased_text {
                Ok(property) => write!(&mut result_directive, "{}", property).unwrap(),
                Err(err) => return Err(err),
            };
        }
        if parts.len() > matches.len() {
            write!(&mut result_directive, "{}", parts[matches.len()]).unwrap();
        }
        Ok(result_directive)
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::{
        entity::EntityId,
        prelude::{Context, Entity},
        story_graph::AliasMap,
    };

    use super::StoryNode;

    #[test]
    fn unalias_directive() {
        const PLAYER: EntityId = 0;
        let node = StoryNode::new().with_directive("{player.name}");
        let mut alias_map = AliasMap::default();
        let context = Context::default();

        let directive = node.directive(&alias_map, &context);
        assert!(directive.is_err());

        alias_map.associate("player".into(), PLAYER);
        let directive = node.directive(&alias_map, &context);
        assert!(directive.is_err());

        let context = Context::default().with_entity(Entity::new(PLAYER));
        let directive = node.directive(&alias_map, &context);
        assert!(directive.is_err());

        let context = Context::default().with_entity(Entity::new(PLAYER).with("name", "My Name"));
        let directive = node.directive(&alias_map, &context).unwrap();
        assert_eq!(directive, "My Name");
    }

    #[test]
    fn multiple_entity_properties() {
        const PLAYER: EntityId = 0;
        let mut alias_map = AliasMap::default();
        alias_map.associate("player".into(), PLAYER);
        let context = Context::default().with_entity(
            Entity::new(PLAYER)
                .with("name", "Umberto")
                .with("level", 1)
                .with("class", "explorer"),
        );

        let node = StoryNode::new().with_directive(
            "Hello, I am {player.name} and I am a level {player.level} {player.class}.",
        );

        let directive = node.directive(&alias_map, &context).unwrap();
        assert_eq!(
            directive,
            "Hello, I am Umberto and I am a level 1 explorer."
        );
    }

    #[test]
    fn multiple_aliases() {
        const PLAYER: EntityId = 0;
        const SHOPKEEP: EntityId = 1;
        let mut alias_map = AliasMap::default();
        alias_map.associate("player".into(), PLAYER);
        alias_map.associate("vendor".into(), SHOPKEEP);
        let context = Context::default()
            .with_entity(
                Entity::new(PLAYER)
                    .with("name", "Umberto")
                    .with("level", 1)
                    .with("class", "explorer"),
            )
            .with_entity(
                Entity::new(SHOPKEEP)
                    .with("name", "Hialda")
                    .with("age", 18.0),
            )
            .with_world_property("location", "Calvinton");

        let node = StoryNode::new().with_directive(
            r#"speak {<a>vendor} {<a>player} "Hello {player.name} the {player.class}! Although I am only {vendor.age} years old, I am the namesake of this {location} shop: {vendor.name}'s Goods!""#,
        );

        let directive = node.directive(&alias_map, &context).unwrap();
        assert_eq!(
            directive,
            r#"speak 1 0 "Hello Umberto the explorer! Although I am only 18 years old, I am the namesake of this Calvinton shop: Hialda's Goods!""#
        );
    }
}
