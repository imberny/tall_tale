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

// #[derive(Serialize, Deserialize)]
#[derive(Default)]
pub struct StoryGraph {
    aliases: Vec<ConstrainedAlias>,
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

    pub fn add_alias<A, C>(&mut self, alias: A, constraints: C)
    where
        A: Into<Alias>,
        C: IntoIterator<Item = Constraint>,
    {
        self.aliases.push(ConstrainedAlias::new(alias, constraints));
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
        toposort(&self.graph, None).map(|_| ()).map_err(|_| {
            self.graph.remove_edge(edge);
            CycleDetected
        })
    }

    // weak edges mean no inheritance in order to prevent cycles
    pub fn connect_weak(&mut self, from: NodeIndex, to: NodeIndex) -> Result<(), CycleDetected> {
        self.weak_edges
            .entry(from)
            .or_insert(Vec::default())
            .push(to);
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

fn collect_tree(node_id: NodeIndex, story_graph: &StoryGraph) -> Node {
    let mut node = Node {
        story: story_graph.get(node_id),
        children: vec![],
        is_leaf: false,
    };

    node.is_leaf = story_graph.all_connections(node_id).is_empty();

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
