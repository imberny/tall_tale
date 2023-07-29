mod constraint;
mod entity;
mod property;
mod query;
mod raconteur;
mod story_graph;
mod story_node;

pub type Integer = i64;
pub type Float = f64;

pub mod prelude {
    pub use crate::{
        constraint::Constraint,
        entity::{Entity, EntityId},
        query::Query,
        raconteur::{Raconteur, StoryId},
        story_graph::StoryGraph,
        story_node::StoryNode,
        Float, Integer,
    };
}
