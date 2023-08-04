#[cfg(test)]
mod story_graph_tests {
    use raconteur::prelude::{Constraint, Entity, StoryGraph, StoryNode, StoryWorld};

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

        graph.start_with(start);

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

    #[test]
    fn traversing_a_graph_of_depth_2() {
        let graph = player_meets_citizen_with_two_outcomes();

        let mut node_index = graph.start();
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
        graph.start_with(a);

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
        graph.start_with(a);
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
        graph.start_with(a);
        let _ = graph.connect(a, b);

        let story_world = StoryWorld::new().with_entity(Entity::new(0));
        let result = graph.alias_candidates(&story_world);

        assert!(result.is_err());
    }
}
