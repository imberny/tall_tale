#[cfg(test)]
mod query_tests {

    use raconteur::prelude::*;

    const GUY_ID: usize = 0;
    const GIRL_ID: usize = 1;

    fn query() -> StoryWorld {
        StoryWorld::new()
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

            graph.add_alias("guy", []);
            graph.add_alias("girl", []);
            let idx = graph.add(
                StoryNode::new()
                    .with_description("low_opinion")
                    .with_relation_constraints(
                        "guy",
                        "girl",
                        [Constraint::is_in_range("opinion", 0..1)],
                    ),
            );

            graph.set_start_node(idx);

            graph
        });

        raconteur
    }

    fn guy_like_girl() -> Raconteur {
        let mut raconteur = Raconteur::new();

        raconteur.insert({
            let mut graph = StoryGraph::new();
            graph.add_alias("guy", []);
            graph.add_alias("girl", []);

            let idx = graph.add(
                StoryNode::new()
                    .with_description("guy_like_girl")
                    .with_relation_constraints(
                        "guy",
                        "girl",
                        [Constraint::is_in_range("opinion", 1..4)],
                    ),
            );

            graph.set_start_node(idx);

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
        let first_story = &story_candidates[0];
        let story_graph = raconteur.get(first_story.id);
        let start_node = story_graph.get(story_graph.start());
        assert_eq!(start_node.description, "guy_like_girl");
    }

    #[test]
    fn many_matches() {
        const MAX_MONEY: Float = 100000.0;
        const PLAYER_ID: usize = 1;
        const BAKER_ID: usize = 2;
        const CUSTOMER_ID: usize = 3;

        let mut raconteur = Raconteur::default();
        // wealthy player
        raconteur.insert({
            let mut graph = StoryGraph::new();
            graph.add_alias(
                "baking_man",
                [
                    Constraint::has("important"),
                    Constraint::equals("job", "baker"),
                ],
            );
            graph.add_alias(
                "player",
                [
                    Constraint::has("player"),
                    Constraint::is_in_range_float("money", 10.0..MAX_MONEY), // TODO: at least, at most?
                ],
            );
            let node_idx = graph.add(
                StoryNode::new().with_world_constraint(Constraint::equals("location", "bakery")),
            );

            graph.set_start_node(node_idx);

            graph
        });

        // Poor player
        raconteur.insert({
            let mut graph = StoryGraph::new();
            graph.add_alias(
                "baking_man",
                [
                    Constraint::has("important"),
                    Constraint::equals("job", "baker"),
                ],
            );
            graph.add_alias(
                "player",
                [
                    Constraint::has("player"),
                    Constraint::is_in_range_float("money", 0.0..20.0),
                ],
            );
            let node_idx = graph.add(
                StoryNode::new().with_world_constraint(Constraint::equals("location", "bakery")),
            );

            graph.set_start_node(node_idx);

            graph
        });

        let query_player_wealthy = StoryWorld::new()
            .with_entities([
                Entity::new(PLAYER_ID)
                    .with("player", "")
                    .with("money", 50.0),
                Entity::new(BAKER_ID)
                    .with("important", "")
                    .with("job", "baker"),
                Entity::new(CUSTOMER_ID),
            ])
            .with_world_property("location", "bakery");

        let stories = raconteur.query(&query_player_wealthy);

        assert_eq!(stories.len(), 1);
        let first_story = &stories[0];
        assert_eq!(first_story.alias_candidates.len(), 1);
        let aliases = &first_story.alias_candidates[0];
        assert_eq!(aliases["player"], PLAYER_ID);
        assert_eq!(aliases["baking_man"], BAKER_ID);

        let query_player_poor = StoryWorld::new()
            .with_entities([
                Entity::new(PLAYER_ID).with("player", "").with("money", 0.0),
                Entity::new(BAKER_ID)
                    .with("important", "")
                    .with("job", "baker"),
                Entity::new(CUSTOMER_ID),
            ])
            .with_world_property("location", "bakery");

        let stories = raconteur.query(&query_player_poor);

        assert_eq!(stories.len(), 1);
        let first_story = &stories[0];
        assert_eq!(first_story.alias_candidates.len(), 1);
        let aliases = &first_story.alias_candidates[0];
        assert_eq!(aliases["player"], PLAYER_ID);
        assert_eq!(aliases["baking_man"], BAKER_ID);

        let query_player_average_wealth = StoryWorld::new()
            .with_entities([
                Entity::new(PLAYER_ID)
                    .with("player", "")
                    .with("money", 15.0),
                Entity::new(BAKER_ID)
                    .with("important", "")
                    .with("job", "baker"),
                Entity::new(CUSTOMER_ID),
            ])
            .with_world_property("location", "bakery");

        let stories = raconteur.query(&query_player_average_wealth);

        assert_eq!(stories.len(), 2);
    }
}
