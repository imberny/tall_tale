use std::{cell::RefCell, rc::Rc};

use itertools::Itertools;

use crate::{
    narrative_world::NarrativeWorld,
    prelude::ScenarioGraph,
    scenario_graph::{AliasMap, ScenarioActionId},
};

pub struct ScenarioChoice {
    id: ScenarioActionId,
    pub description: String,
    pub directive: String,
}

pub struct Scenario {
    id: usize,
    pub weight: f32,
    graph: Rc<ScenarioGraph>,
    pub alias_map: AliasMap,
    current_action: RefCell<ScenarioActionId>,
}

impl Scenario {
    pub fn new(id: usize, graph: Rc<ScenarioGraph>, alias_map: AliasMap) -> Self {
        let start_action = graph.start();
        Self {
            id,
            weight: graph.num_alias_constraints() as f32,
            graph,
            alias_map,
            current_action: RefCell::new(start_action),
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn is_completed(&self) -> bool {
        self.graph.connections(self.current_node()).is_empty()
    }

    pub fn choices(&self, narrative_world: &NarrativeWorld) -> Vec<ScenarioChoice> {
        self.graph
            .connections(self.current_node())
            .into_iter()
            .map(|action_id| {
                let action = self.graph.get(action_id.into());
                ScenarioChoice {
                    id: action_id.into(),
                    description: action.description.clone(),
                    directive: action.directive(&self.alias_map, narrative_world).unwrap(),
                }
            })
            .collect_vec()
    }

    fn current_node(&self) -> petgraph::stable_graph::NodeIndex {
        self.current_action.clone().take().into()
    }

    pub fn choose(&mut self, choice: ScenarioChoice) {}
}
