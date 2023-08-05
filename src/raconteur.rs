use itertools::Itertools;

use crate::{
    context::Context,
    story_graph::{AliasMap, StoryGraph},
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct StoryId(usize);

pub struct StoryCandidate {
    pub id: StoryId,
    pub alias_candidates: Vec<AliasMap>,
}

// #[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct Raconteur {
    stories: Vec<StoryGraph>,
}

impl Raconteur {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, story_graph: StoryGraph) {
        self.stories.push(story_graph);
    }

    pub fn query(&self, context: &Context) -> Vec<StoryCandidate> {
        // go through list of story beats, discarding those whose constraints aren't satisfied

        self.stories
            .iter()
            .enumerate()
            .filter(|&(index, _)| context.is_included(&StoryId(index)))
            .filter_map(|(story_idx, story_graph)| {
                let result = story_graph.alias_candidates(context);

                result.ok().map(|alias_candidates| StoryCandidate {
                    id: StoryId(story_idx),
                    alias_candidates,
                })
            })
            .collect_vec()
    }

    pub fn get(&self, story_id: StoryId) -> &StoryGraph {
        &self.stories[story_id.0]
    }
}

#[cfg(test)]
mod unit_tests {
    use crate::prelude::{Context, StoryGraph, StoryNode};

    use super::Raconteur;

    #[test]
    fn a_story_can_be_excluded_from_the_query_result() {
        let mut raconteur = Raconteur::new();
        raconteur.insert({
            let mut graph = StoryGraph::new();
            let a = graph.add(StoryNode::new());
            graph.set_start_node(a);
            graph
        });

        let mut context = Context::new();

        let stories = raconteur.query(&context);
        assert!(!stories.is_empty());
        context.exclude(&[stories[0].id]);
        let stories = raconteur.query(&context);
        assert!(stories.is_empty());
    }
}
