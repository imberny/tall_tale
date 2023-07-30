use petgraph::{
    algo::{toposort, DfsSpace},
    prelude::{Graph, NodeIndex},
    visit::{Dfs, GraphBase, Visitable},
};
use std::{error::Error, fmt};

use crate::story_node::StoryNode;

#[derive(Debug)]
pub struct CycleDetected;

impl fmt::Display for CycleDetected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cycle detected")
    }
}

impl Error for CycleDetected {}

struct WeakEdge(NodeIndex, NodeIndex);

// #[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct StoryGraph {
    start_index: NodeIndex,
    graph: Graph<StoryNode, f64>,
    weak_edges: Vec<WeakEdge>,
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

    pub fn connect(&mut self, from: NodeIndex, to: NodeIndex) -> Result<(), CycleDetected> {
        self.connect_weight(from, to, 0.0)
    }

    pub fn connect_weight(
        &mut self,
        from: NodeIndex,
        to: NodeIndex,
        weight: f64,
    ) -> Result<(), CycleDetected> {
        let edge = self.graph.add_edge(from, to, weight);
        let result = toposort(&self.graph, None).map(|_| ()).map_err(|_| {
            self.graph.remove_edge(edge);
            CycleDetected
        });

        if result.is_ok() {
            // connected node inherits aliases, constraints, etc
        }
        result
    }

    //
    pub fn connect_weak(&mut self, from: NodeIndex, to: NodeIndex) {
        self.weak_edges.push(WeakEdge(from, to));
    }
}
