use petgraph::prelude::{Graph, NodeIndex};

use crate::story_node::StoryNode;

// #[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct StoryGraph {
    start_index: NodeIndex,
    graph: Graph<StoryNode, f64>,
}

impl StoryGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&self) -> &StoryNode {
        &self.graph[self.start_index]
    }

    pub fn start_with(&mut self, node_index: NodeIndex) {
        self.start_index = node_index;
    }

    pub fn add(&mut self, story_node: StoryNode) -> NodeIndex {
        self.graph.add_node(story_node)
    }

    pub fn connect(&mut self, from: NodeIndex, to: NodeIndex) {
        self.connect_weight(from, to, 0.0);
    }

    pub fn connect_weight(&mut self, from: NodeIndex, to: NodeIndex, weight: f64) {
        self.graph.add_edge(from, to, weight);
    }
}
