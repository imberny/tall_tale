mod constraint;
mod context;
mod entity;
mod property;
mod raconteur;
mod story_graph;
mod story_node;

pub type Integer = i64;
pub type Float = f64;

pub mod prelude {
    pub use crate::{
        constraint::Constraint,
        context::Context,
        entity::Entity,
        property::PropertyName,
        raconteur::{Raconteur, StoryCandidate, StoryId},
        story_graph::StoryGraph,
        story_node::StoryNode,
        Float, Integer,
    };
}
