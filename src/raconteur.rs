use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;
use petgraph::prelude::NodeIndex;

use crate::{
    entity::EntityId, prelude::StoryNode, story_graph::StoryGraph, story_node::AliasCandidates,
    story_world::StoryWorld,
};

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
                struct Node<'a> {
                    pub story: &'a StoryNode,
                    pub children: Vec<Node<'a>>,
                }

                fn collect_tree(node_id: NodeIndex, story_graph: &StoryGraph) -> Node {
                    let mut node = Node {
                        story: story_graph.get(node_id),
                        children: vec![],
                    };

                    for child_id in story_graph.connections(node_id) {
                        node.children.push(collect_tree(child_id, story_graph));
                    }

                    node
                }

                // returns list of indices of valid bindings
                fn valid_alias_permutations(
                    node: &Node,
                    story_world: &StoryWorld,
                    alias_binding_permutations: &[HashMap<String, EntityId>],
                    parent_valid_indices: &HashSet<usize>,
                ) -> HashSet<usize> {
                    if !node.story.are_world_constraints_satisfied(story_world) {
                        return HashSet::default();
                    }

                    let valid_indices: HashSet<_> = alias_binding_permutations
                        .iter()
                        .enumerate()
                        .filter(|(index, permutation)| {
                            parent_valid_indices.contains(index)
                                && node
                                    .story
                                    .are_relation_constraints_satisfied(story_world, permutation)
                        })
                        .map(|(index, _)| index)
                        .collect();

                    if node.children.is_empty() {
                        return valid_indices;
                    }

                    let mut final_valid_indices = HashSet::default();
                    for child_node in &node.children {
                        let child_valid_indices = valid_alias_permutations(
                            child_node,
                            story_world,
                            alias_binding_permutations,
                            &valid_indices,
                        );
                        final_valid_indices.extend(child_valid_indices);
                    }

                    final_valid_indices
                }

                // assert at least one valid alias permutation
                let permutations = story_graph.alias_permutations(story_world);

                if permutations.is_empty() {
                    return None;
                }

                let node = collect_tree(story_graph.start(), story_graph);
                let permutation_indices = HashSet::from_iter(0..permutations.len());
                let valid_permutation_indices = valid_alias_permutations(
                    &node,
                    story_world,
                    &permutations,
                    &permutation_indices,
                );

                if valid_permutation_indices.is_empty() {
                    return None;
                }

                let valid_permutations = permutations
                    .into_iter()
                    .enumerate()
                    .filter_map(|(index, permutation)| {
                        valid_permutation_indices
                            .contains(&index)
                            .then_some(permutation)
                    })
                    .collect_vec();

                Some(StoryCandidate {
                    id: StoryId(story_idx),
                    alias_candidates: valid_permutations,
                })
            })
            .collect_vec()
    }

    pub fn get(&self, story_id: StoryId) -> &StoryGraph {
        &self.stories[story_id.0]
    }
}
