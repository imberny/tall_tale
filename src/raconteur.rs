use std::rc::Rc;

use itertools::Itertools;

use crate::{
    narrative_world::NarrativeWorld,
    prelude::Scenario,
    scenario_graph::{AliasMap, ScenarioGraph},
};

// #[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct Raconteur {
    stories: Vec<Rc<ScenarioGraph>>,
}

impl Raconteur {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, scenario_graph: ScenarioGraph) {
        self.stories.push(Rc::new(scenario_graph));
    }

    pub fn query(&self, context: &NarrativeWorld) -> Vec<Scenario> {
        // go through list of story beats, discarding those whose constraints aren't satisfied

        self.stories
            .iter()
            .enumerate()
            // .filter(|&(index, _)| context.is_included(&StoryId(index)))
            .filter_map(|(index, scenario_graph)| {
                let result = scenario_graph.alias_candidates(context);

                result.ok().map(|alias_candidates| {
                    alias_candidates
                        .iter()
                        .map(|alias_map| {
                            Scenario::new(index, Rc::clone(scenario_graph), alias_map.clone())
                        })
                        .collect_vec()
                })
            })
            .flatten()
            .collect_vec()
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::prelude::{NarrativeWorld, ScenarioAction, ScenarioGraph};

    use super::Raconteur;

    #[test]
    fn a_story_can_be_excluded_from_the_query_result() {
        let mut raconteur = Raconteur::new();
        raconteur.insert({
            let mut graph = ScenarioGraph::new();
            let a = graph.add(ScenarioAction::new());
            graph.set_start_node(a);
            graph
        });

        let mut context = NarrativeWorld::new();

        let stories = raconteur.query(&context);
        assert!(!stories.is_empty());
        // context.exclude(&[stories[0].id]);
        let stories = raconteur.query(&context);
        assert!(stories.is_empty());
    }
}
