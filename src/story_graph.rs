use itertools::Itertools;
use petgraph::{
    algo::toposort,
    prelude::{Graph, NodeIndex},
};
use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt,
};

use crate::{
    entity::EntityId,
    prelude::{Constraint, StoryWorld},
    story_node::{Alias, ConstrainedAlias, StoryNode},
};

#[derive(Debug)]
pub struct CycleDetected;
impl fmt::Display for CycleDetected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cycle detected")
    }
}
impl Error for CycleDetected {}

#[derive(Debug)]
pub struct ConstraintsNotSatisfied;
impl fmt::Display for ConstraintsNotSatisfied {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Constraints not satisfied")
    }
}
impl Error for ConstraintsNotSatisfied {}

#[derive(Default, Clone, Copy)]
pub struct StoryNodeId(NodeIndex);

// #[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct StoryGraph {
    aliases: Vec<ConstrainedAlias>,
    start_id: StoryNodeId,
    graph: Graph<StoryNode, f64>,
    weak_edges: HashMap<NodeIndex, Vec<NodeIndex>>,
}

impl StoryGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start(&self) -> StoryNodeId {
        self.start_id
    }

    pub fn add_alias<A, C>(&mut self, alias: A, constraints: C)
    where
        A: Into<Alias>,
        C: IntoIterator<Item = Constraint>,
    {
        self.aliases.push(ConstrainedAlias::new(alias, constraints));
    }

    pub fn get(&self, node_id: StoryNodeId) -> &StoryNode {
        &self.graph[node_id.0]
    }

    pub(crate) fn connections(&self, node_id: NodeIndex) -> Vec<NodeIndex> {
        self.graph.neighbors(node_id).collect()
    }

    pub(crate) fn all_connections(&self, node_id: NodeIndex) -> Vec<NodeIndex> {
        let mut connections = self.graph.neighbors(node_id).collect_vec();
        connections.extend(self.weak_edges.get(&node_id).unwrap_or(&Vec::default()));
        connections
    }

    pub fn next(
        &self,
        node_id: StoryNodeId,
        story_world: &StoryWorld,
        alias_map: &HashMap<Alias, EntityId>,
    ) -> Vec<StoryNodeId> {
        self.all_connections(node_id.0)
            .into_iter()
            .filter(|&index| {
                let node = &self.graph[index];
                node.are_world_constraints_satisfied(story_world)
                    && node.are_relation_constraints_satisfied(story_world, alias_map)
            })
            .map(StoryNodeId)
            .collect()
    }

    pub fn set_start_node(&mut self, node_id: StoryNodeId) {
        self.start_id = node_id;
    }

    pub fn add(&mut self, story_node: StoryNode) -> StoryNodeId {
        let index = self.graph.add_node(story_node);
        StoryNodeId(index)
    }

    pub fn connect(&mut self, from: StoryNodeId, to: StoryNodeId) -> Result<(), CycleDetected> {
        self.connect_weight(from, to, 0.0)
    }

    pub fn connect_weight(
        &mut self,
        parent: StoryNodeId,
        child: StoryNodeId,
        weight: f64,
    ) -> Result<(), CycleDetected> {
        let edge = self.graph.add_edge(parent.0, child.0, weight);
        toposort(&self.graph, None).map(|_| ()).map_err(|_| {
            self.graph.remove_edge(edge);
            CycleDetected
        })
    }

    // weak edges mean no inheritance in order to prevent cycles
    pub fn connect_weak(
        &mut self,
        from: StoryNodeId,
        to: StoryNodeId,
    ) -> Result<(), CycleDetected> {
        self.weak_edges
            .entry(from.0)
            .or_insert(Vec::default())
            .push(to.0);
        Ok(())
    }

    pub fn alias_candidates(
        &self,
        story_world: &StoryWorld,
    ) -> Result<Vec<HashMap<Alias, EntityId>>, ConstraintsNotSatisfied> {
        if self.aliases.is_empty() {
            return Ok(vec![]);
        }

        // assert at least one valid alias permutation
        let permutations = self.alias_permutations(story_world);

        if permutations.is_empty() {
            return Err(ConstraintsNotSatisfied);
        }

        let node = collect_tree(self.start(), self);
        let permutation_indices = HashSet::from_iter(0..permutations.len());
        let valid_permutation_indices =
            valid_alias_permutations(&node, story_world, &permutations, &permutation_indices);

        let valid_permutations = permutations
            .into_iter()
            .enumerate()
            .filter_map(|(index, permutation)| {
                valid_permutation_indices
                    .contains(&index)
                    .then_some(permutation)
            })
            .collect_vec();

        let any_valid_permutation = !valid_permutations.is_empty();
        any_valid_permutation
            .then_some(valid_permutations)
            .ok_or(ConstraintsNotSatisfied)
    }

    // return list of possible alias permutations
    // Doesn't validate relation constraints, a those can vary from node to node and thus affect which choices are available
    fn alias_permutations(&self, story_world: &StoryWorld) -> Vec<HashMap<Alias, EntityId>> {
        let alias_candidates: HashMap<_, _> = self
            .aliases
            .iter()
            .map(|constrained_alias| {
                let valid_entities = story_world
                    .entities
                    .iter()
                    .filter(|entity| constrained_alias.is_satisfied_by(&entity.properties))
                    .map(|entity| entity.id())
                    .collect_vec();
                (constrained_alias.alias().clone(), valid_entities)
            })
            .collect();

        if alias_candidates.is_empty() {
            return vec![];
        }

        let (first_alias, first_candidates) = alias_candidates.iter().next().unwrap();
        let mut permutations = first_candidates
            .iter()
            .map(|id| HashMap::from([(*id, first_alias)]))
            .collect_vec();

        for (alias, candidates) in alias_candidates.iter().skip(1) {
            permutations = permutations
                .into_iter()
                .cartesian_product(candidates.iter().cloned())
                .map(|(mut ids, id)| {
                    ids.insert(id, alias);
                    ids
                })
                .collect();
        }
        permutations.retain(|permutation| permutation.len() == self.aliases.len());

        let mut alias_permutations = vec![];
        for permutation in permutations {
            let mut alias_permutation = HashMap::default();
            for (entity, alias) in permutation {
                alias_permutation.insert(alias.clone(), entity);
            }
            alias_permutations.push(alias_permutation);
        }

        alias_permutations
    }
}

struct Node<'a> {
    pub story: &'a StoryNode,
    pub children: Vec<Node<'a>>,
    pub is_leaf: bool,
}

fn collect_tree(node_id: StoryNodeId, story_graph: &StoryGraph) -> Node {
    let mut node = Node {
        story: story_graph.get(node_id),
        children: vec![],
        is_leaf: false,
    };

    node.is_leaf = story_graph.all_connections(node_id.0).is_empty();

    for child_id in story_graph.connections(node_id.0) {
        node.children
            .push(collect_tree(StoryNodeId(child_id), story_graph));
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
        if node.is_leaf {
            return valid_indices;
        } else {
            return HashSet::default();
        }
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

#[cfg(test)]
mod unit_tests {
    use crate::prelude::{Constraint, Entity, StoryWorld};

    use crate::{story_graph::StoryGraph, story_node::StoryNode};

    fn player_meets_citizen_with_two_outcomes() -> StoryGraph {
        let mut graph = StoryGraph::new();

        graph.add_alias("player", [Constraint::has("protagonist")]);
        graph.add_alias("citizen", []);

        let start = graph.add(
            StoryNode::new()
                .with_description("player talks to a new citizen")
                .with_relation_constraints(
                    "player",
                    "citizen",
                    [Constraint::has_not("previously_met")],
                )
                .with_directive("add relation player citizen previously_met"),
        );

        graph.set_start_node(start);

        let citizen_greeting = graph.add(
            StoryNode::new()
                .with_description("citizen greets player")
                .with_directive(r#"citizen says "Long days and pleasant nights.""#),
        );

        let _ = graph.connect(start, citizen_greeting);

        let ask_for_directions = graph.add(
            StoryNode::new()
                .with_description("player asks for directions")
                .with_directive(r#"player says "Could you tell me where I could find...""#),
        );

        let _ = graph.connect(citizen_greeting, ask_for_directions);

        let goodbye = graph.add(
            StoryNode::new()
                .with_description("player quits dialogue")
                .with_directive(r#"player says "Goodbye, sai.""#),
        );

        let _ = graph.connect(citizen_greeting, goodbye);

        graph
    }

    #[test]
    fn graph_cycle() {
        let mut graph = StoryGraph::new();

        let a = graph.add(StoryNode::new());
        let b = graph.add(StoryNode::new());
        let c = graph.add(StoryNode::new());

        let result = graph.connect(a, b);
        assert!(result.is_ok());
        let result = graph.connect(b, c);
        assert!(result.is_ok());
        let result = graph.connect(c, a);
        assert!(result.is_err());
    }

    #[test]
    fn graph_cycle_weak() {
        let mut graph = StoryGraph::new();

        let a = graph.add(StoryNode::new());
        let b = graph.add(StoryNode::new());
        let c = graph.add(StoryNode::new());

        let result = graph.connect(a, b);
        assert!(result.is_ok());
        let result = graph.connect(b, c);
        assert!(result.is_ok());
        let result = graph.connect_weak(c, a);
        assert!(result.is_ok());
    }

    #[test]
    fn traversing_a_graph_of_depth_2() {
        let graph = player_meets_citizen_with_two_outcomes();

        let mut node_index = graph.start().0;
        let mut nodes_traversed = 0;
        loop {
            node_index = graph.connections(node_index)[0];
            nodes_traversed += 1;
            if graph.connections(node_index).is_empty() {
                break;
            }
        }

        assert_eq!(nodes_traversed, 2);
    }

    #[test]
    fn a_graph_with_no_leaf_node_is_err() {
        let mut graph = StoryGraph::new();
        graph.add_alias("person", []);
        let a = graph.add(StoryNode::new());
        let b = graph.add(StoryNode::new());
        // TODO: this particular case should be caught when building the graph
        let _ = graph.connect(a, b);
        let _ = graph.connect_weak(b, a);

        let story_world = StoryWorld::new().with_entity(Entity::new(0));
        let result = graph.alias_candidates(&story_world);

        assert!(result.is_err());
    }

    #[test]
    fn a_graph_with_no_aliases_is_ok() {
        let mut graph = StoryGraph::new();
        let a = graph.add(StoryNode::new());
        graph.set_start_node(a);

        let story_world = StoryWorld::new().with_entity(Entity::new(0));
        let result = graph.alias_candidates(&story_world);

        assert!(result.is_ok());
    }

    #[test]
    fn a_graph_needs_at_least_one_reachable_leaf_node() {
        let mut graph: StoryGraph = StoryGraph::new();
        graph.add_alias("person", []);

        let a = graph.add(StoryNode::new());
        let b = graph.add(StoryNode::new());
        graph.set_start_node(a);
        let _ = graph.connect(a, b);

        let story_world = StoryWorld::new().with_entity(Entity::new(0));
        let result = graph.alias_candidates(&story_world);

        assert!(result.is_ok());
        let alias_permutations = result.unwrap();
        assert_eq!(alias_permutations.len(), 1);
    }

    #[test]
    fn a_graph_with_no_reachable_leaf_node_is_err() {
        let mut graph: StoryGraph = StoryGraph::new();
        graph.add_alias("person", []);

        let a = graph.add(StoryNode::new());
        let b =
            graph.add(StoryNode::new().with_world_constraint(Constraint::has("some constraint")));
        graph.set_start_node(a);
        let _ = graph.connect(a, b);

        let story_world = StoryWorld::new().with_entity(Entity::new(0));
        let result = graph.alias_candidates(&story_world);

        assert!(result.is_err());
    }

    #[test]
    fn single_alias_candidate_permutation_possible() {
        const PROTAGONIST: usize = 0;
        const NEW_CITIZEN: usize = 1;
        const KNOWN_CITIZEN: usize = 2;
        let story_world = StoryWorld::new()
            .with_entities([
                Entity::new(PROTAGONIST).with("protagonist", ""),
                Entity::new(NEW_CITIZEN),
                Entity::new(KNOWN_CITIZEN),
            ])
            .with_relation(PROTAGONIST, KNOWN_CITIZEN, "previously_met", "");

        let graph = player_meets_citizen_with_two_outcomes();

        let result = graph.alias_candidates(&story_world);
        assert!(result.is_ok());
        let permutations = result.unwrap();
        assert_eq!(permutations.len(), 1);
        let aliases = &permutations[0];
        assert_eq!(aliases["player"], PROTAGONIST);
        assert_eq!(aliases["citizen"], NEW_CITIZEN);
    }
}
