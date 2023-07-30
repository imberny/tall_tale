use itertools::Itertools;

use crate::{story_graph::StoryGraph, story_node::AliasCandidates, story_world::StoryWorld};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct StoryId(usize);

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

    // Returns a pair of valid story beat with its list of valid aliased entities
    // inner vec is a list of permutations of indices. first index is for first alias, etc.
    pub fn query(&self, story_world: &StoryWorld) -> Vec<(StoryId, Vec<AliasCandidates>)> {
        // go through list of story beats, discarding those whose constraints aren't satisfied

        self.stories
            .iter()
            .enumerate()
            .filter_map(|(story_idx, story_graph)| {
                // go through children, look for at least one valid path to a childless node
                let mut alias_candidates = Vec::default();
                let mut node_indices = vec![story_graph.start()];
                let mut success = false;
                while !node_indices.is_empty() {
                    let node_index = node_indices[0];
                    node_indices = node_indices[1..].to_vec();
                    let node = story_graph.get(node_index);

                    // validate node
                    // TODO: problem... since aliases are inherited by child nodes, we're generating duplicate matches.
                    //		Should I pass in a structure of matched aliases? Or should inherited constraints be kept apart?
                    if let Ok(candidates) = node.try_matching_aliases(story_world) {
                        alias_candidates.extend(candidates);
                        node_indices.extend(story_graph.connections(node_index));
                        if story_graph.all_connections(node_index).is_empty() {
                            // success! at least one reachable leaf node
                            success = true;
                            break;
                        }
                    }
                }
                success.then_some((StoryId(story_idx), alias_candidates))
            })
            .collect_vec()
    }

    pub fn get(&self, story_id: StoryId) -> &StoryGraph {
        &self.stories[story_id.0]
    }
}
