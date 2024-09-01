mod constraint;
mod entity;
mod instruction;
mod narrative_world;
mod property;
mod raconteur;
mod relationship;
mod scenario;
mod scenario_action;
mod scenario_graph;
mod schema;

pub type Int = i64;
pub type Real = f64;

pub mod prelude {
    pub use crate::{
        constraint::Constraint, entity::Entity, narrative_world::NarrativeWorld,
        property::PropertyName, raconteur::Raconteur, scenario::Scenario,
        scenario_action::ScenarioAction, scenario_graph::ScenarioGraph, Int, Real,
    };
}
