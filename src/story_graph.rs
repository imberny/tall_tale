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

    pub fn get(&self, node_id: NodeIndex) -> &StoryNode {
        &self.graph[node_id]
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
        parent: NodeIndex,
        child: NodeIndex,
        weight: f64,
    ) -> Result<(), CycleDetected> {
        let edge = self.graph.add_edge(parent, child, weight);
        let result = toposort(&self.graph, None).map(|_| ()).map_err(|_| {
            self.graph.remove_edge(edge);
            CycleDetected
        });

        if result.is_ok() {
            let parent = &self.graph[parent];
            let aliases = parent.aliases.clone();
            let relations = parent.relation_constraints.clone();
            let world_constraints = parent.world_constraints.clone();
            self.graph[child].aliases.extend(aliases);
            self.graph[child].relation_constraints.extend(relations);
            self.graph[child]
                .world_constraints
                .extend(world_constraints);
        }
        result
    }

    //
    pub fn connect_weak(&mut self, from: NodeIndex, to: NodeIndex) -> Result<(), CycleDetected> {
        self.weak_edges.push(WeakEdge(from, to));
        Ok(())
    }
}
