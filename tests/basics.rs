#[cfg(test)]
mod tests {

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
            let idx = graph.add(
                StoryNode::new()
                    .with_description("low_opinion")
                    .with_alias_constraints("guy", [])
                    .with_alias_constraints("girl", [])
                    .with_relation_constraints(
                        "guy",
                        "girl",
                        [Constraint::is_in_range("opinion", 0..1)],
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
                        [Constraint::is_in_range("opinion", 1..4)],
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
        raconteur.insert({
            let mut graph = StoryGraph::new();
            let node_idx = graph.add(
                StoryNode::new()
                    .with_alias_constraints(
                        "baking_man",
                        [
                            Constraint::has("important"),
                            Constraint::equals("job", "baker"),
                        ],
                    )
                    .with_alias_constraints(
                        "player",
                        [
                            Constraint::has("player"),
                            Constraint::is_in_range_float("money", 10.0..MAX_MONEY), // TODO: at least, at most?
                        ],
                    )
                    .with_world_constraint(Constraint::equals("location", "bakery")),
            );

            graph.start_with(node_idx);

            graph
        });

        raconteur.insert({
            let mut graph = StoryGraph::new();
            let node_idx = graph.add(
                StoryNode::new()
                    .with_alias_constraints(
                        "baking_man",
                        [
                            Constraint::has("important"),
                            Constraint::equals("job", "baker"),
                        ],
                    )
                    .with_alias_constraints(
                        "player",
                        [
                            Constraint::has("player"),
                            Constraint::is_in_range_float("money", 0.0..20.0), // TODO: at least, at most?
                        ],
                    )
                    .with_world_constraint(Constraint::equals("location", "bakery")),
            );

            graph.start_with(node_idx);

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
        let (_story_id, alias_candidates) = &stories[0];
        assert_eq!(alias_candidates.len(), 1);
        let aliases = &alias_candidates[0];
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
        let (_story_id, alias_candidates) = &stories[0];
        assert_eq!(alias_candidates.len(), 1);
        let aliases = &alias_candidates[0];
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
