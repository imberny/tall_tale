#[cfg(test)]
mod tests {
    use raconteur::prelude::{Constraint, Entity, Raconteur, StoryGraph, StoryNode, StoryWorld};

    #[test]
    fn story_node_with_one_child() {
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
        const CITIZEN: usize = 1;
        let story_world = StoryWorld::new().with_entities([
            Entity::new(PROTAGONIST).with("protagonist", ""),
            Entity::new(CITIZEN),
        ]);

        let candidates = raconteur.query(&story_world);
    }
}
