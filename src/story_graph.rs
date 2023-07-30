use itertools::Itertools;
use petgraph::{
    algo::toposort,
    prelude::{Graph, NodeIndex},
};
use std::{collections::HashMap, error::Error, fmt};

use crate::story_node::StoryNode;

#[derive(Debug)]
pub struct CycleDetected;
impl fmt::Display for CycleDetected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cycle detected")
    }
}
impl Error for CycleDetected {}

// #[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct StoryGraph {
    start_index: NodeIndex,
    graph: Graph<StoryNode, f64>,
    weak_edges: HashMap<NodeIndex, Vec<NodeIndex>>,
}

impl StoryGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&self) -> NodeIndex {
        self.start_index
    }

    pub fn get(&self, node_id: NodeIndex) -> &StoryNode {
        &self.graph[node_id]
    }

    pub fn connections(&self, node_id: NodeIndex) -> Vec<NodeIndex> {
        self.graph.neighbors(node_id).collect()
    }

    pub fn all_connections(&self, node_id: NodeIndex) -> Vec<NodeIndex> {
        let mut connections = self.graph.neighbors(node_id).collect_vec();
        connections.extend(self.weak_edges.get(&node_id).unwrap_or(&Vec::default()));
        connections
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
            // inherit constraints
            let parent = &self.graph[parent];
            let aliases = parent.constraints.aliases.clone();
            let relations = parent.constraints.relation_constraints.clone();
            let world_constraints = parent.constraints.world_constraints.clone();
            self.graph[child]
                .inherited_constraints
                .aliases
                .extend(aliases);
            self.graph[child]
                .inherited_constraints
                .relation_constraints
                .extend(relations);
            self.graph[child]
                .inherited_constraints
                .world_constraints
                .extend(world_constraints);
        }
        result
    }

    // weak edges mean no inheritance in order to prevent cycles
    pub fn connect_weak(&mut self, from: NodeIndex, to: NodeIndex) -> Result<(), CycleDetected> {
        self.weak_edges
            .entry(from)
            .or_insert(Vec::default())
            .push(to);
        Ok(())
    }
}
