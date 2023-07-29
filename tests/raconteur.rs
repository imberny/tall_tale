#[cfg(test)]
mod tests {
    use std::ops::Range;

    use raconteur::prelude::*;

    const GUY_ID: EntityId = EntityId::new(0);
    const GIRL_ID: EntityId = EntityId::new(1);

    fn query() -> Query {
        Query::new()
            .with_entities([
                Entity::new(GUY_ID).with("name", "Bertrand").with("age", 30),
                Entity::new(GIRL_ID)
                    .with("name", "Juliette")
                    .with("age", 32),
            ])
            .with_relation(GUY_ID, GIRL_ID, "opinion", 2)
    }

    fn guy_no_like_girl() -> Raconteur {
        let mut raconteur = Raconteur::new();
        raconteur.insert({
            let mut graph = StoryGraph::new();
            let idx = graph.add(
                StoryNode::new()
                    .with_description("low_opinion")
                    .with_alias_constraints("guy", [])
                    .with_alias_constraints("girl", [])
                    .with_relation_constraints(
                        "guy",
                        "girl",
                        [Constraint::is_in_range(
                            "opinion",
                            Range { start: 0, end: 1 },
                        )],
                    ),
            );

            graph.start_with(idx);

            graph
        });

        raconteur
    }

    fn guy_like_girl() -> Raconteur {
        let mut raconteur = Raconteur::new();

        raconteur.insert({
            let mut graph = StoryGraph::new();
            let idx = graph.add(
                StoryNode::new()
                    .with_description("guy_like_girl")
                    .with_alias_constraints("guy", [])
                    .with_alias_constraints("girl", [])
                    .with_relation_constraints(
                        "guy",
                        "girl",
                        [Constraint::is_in_range(
                            "opinion",
                            Range { start: 1, end: 4 },
                        )],
                    ),
            );

            graph.start_with(idx);

            graph
        });

        raconteur
    }

    #[test]
    fn no_match() {
        let raconteur = guy_no_like_girl();
        let story_candidates = raconteur.query(&query());
        assert!(story_candidates.is_empty());
    }

    #[test]
    fn a_match() {
        let raconteur: Raconteur = guy_like_girl();
        let story_candidates = raconteur.query(&query());
        let (story_id, _alias_candidates) = &story_candidates[0];
        let story_graph = raconteur.get(*story_id);
        let story_node = story_graph.start();
        assert_eq!(story_node.description, "guy_like_girl");
    }

    #[test]
    fn many_matches() {
        let mut raconteur = Raconteur::default();
        raconteur.insert({
            let mut graph = StoryGraph::new();
            let node_idx = graph.add(StoryNode::new().with_alias_constraints(
                "baker",
                [
                    Constraint::has("important"),
                    Constraint::equals("job", "baker"),
                ],
            ));

            graph.start_with(node_idx);

            graph
        });

        // let mut query = Query::
    }
}
