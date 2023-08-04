mod constraint;
mod entity;
mod property;
mod raconteur;
mod story_graph;
mod story_node;
mod story_world;

pub type Integer = i64;
pub type Float = f64;

pub mod prelude {
    pub use crate::{
        constraint::Constraint,
        entity::Entity,
        property::PropertyName,
        raconteur::{Raconteur, StoryCandidate, StoryId},
        story_graph::StoryGraph,
        story_node::StoryNode,
        story_world::StoryWorld,
        Float, Integer,
    };
}
