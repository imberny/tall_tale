use itertools::Itertools;

use crate::{story_graph::StoryGraph, story_node::AliasCandidates, story_world::StoryWorld};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct StoryId(usize);

pub struct StoryCandidate {
    pub id: StoryId,
    pub alias_candidates: Vec<AliasCandidates>,
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

    pub fn query(&self, story_world: &StoryWorld) -> Vec<StoryCandidate> {
        // go through list of story beats, discarding those whose constraints aren't satisfied

        self.stories
            .iter()
            .enumerate()
            .filter_map(|(story_idx, story_graph)| {
                let alias_candidates = story_graph.alias_candidates(story_world);

                if alias_candidates.is_empty() {
                    return None;
                }

                Some(StoryCandidate {
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
