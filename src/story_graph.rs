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

#[derive(Default)]
pub struct AliasPermutation {
    alias_to_id: HashMap<Alias, EntityId>,
    id_to_alias: HashMap<EntityId, Alias>,
}
impl AliasPermutation {
    pub fn id(&self, alias: &Alias) -> Option<&EntityId> {
        self.alias_to_id.get(alias)
    }
    pub fn alias(&self, id: &EntityId) -> Option<&Alias> {
        self.id_to_alias.get(id)
    }
    pub fn associate(&mut self, alias: &Alias, id: &EntityId) {
        self.alias_to_id.insert(alias.clone(), *id);
        self.id_to_alias.insert(*id, alias.clone());
    }
}

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

    // return list of possible alias permutations
    // Doesn't validate relation constraints, a those can vary from node to node and thus affect which choices are available
    pub fn alias_permutations(&self, story_world: &StoryWorld) -> Vec<HashMap<Alias, EntityId>> {
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
