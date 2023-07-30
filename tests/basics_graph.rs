#[cfg(test)]
mod tests {
    use raconteur::prelude::{Constraint, Entity, Raconteur, StoryGraph, StoryNode, StoryWorld};

    #[test]
    fn story_node_traversal() {
        let mut raconteur = Raconteur::new();
        raconteur.insert({
            let mut graph = StoryGraph::new();

            let start = graph.add(
                StoryNode::new()
                    .with_description("player talks to a new citizen")
                    .with_alias_constraints("player", [Constraint::has("protagonist")])
                    .with_alias_constraints("citizen", [])
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
        });

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
        let candidates = raconteur.query(&story_world);
        assert_eq!(candidates.len(), 1);
        let (story_id, alias_candidates) = &candidates[0];
        assert_eq!(alias_candidates.len(), 1);
        let aliases = &alias_candidates[0];
        assert_eq!(aliases["player"], PROTAGONIST);
        assert_eq!(aliases["citizen"], NEW_CITIZEN);
        let story_graph = raconteur.get(*story_id);

        let mut node_index = story_graph.start();
        let mut nodes_traversed = 0;
        loop {
            node_index = story_graph.connections(node_index)[0];
            nodes_traversed += 1;
            if story_graph.connections(node_index).is_empty() {
                break;
            }
        }

        assert_eq!(nodes_traversed, 2);
    }

    #[test]
    fn graph_needs_at_least_one_leaf_node() {
        let mut raconteur = Raconteur::new();
        raconteur.insert({
            let mut graph = StoryGraph::new();

            let a = graph.add(StoryNode::new());
            let b = graph.add(StoryNode::new());
            let _ = graph.connect(a, b);
            let _ = graph.connect_weak(b, a);

            graph
        });

        let story_candidates = raconteur.query(&StoryWorld::new());

        assert!(story_candidates.is_empty())
    }
}
