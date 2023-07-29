mod constraint;
mod entity;
mod property;
mod query;
mod raconteur;
mod story_graph;
mod story_node;

pub mod prelude {
    // TODO: restrict pub types to strict minimum
    pub use crate::{
        constraint::{PropertyConstraint, RelationConstraint},
        entity::{Entity, EntityId},
        property::{Property, PropertyMap},
        query::*,
        raconteur::*,
        story_graph::StoryGraph,
        story_node::*,
    };
}
