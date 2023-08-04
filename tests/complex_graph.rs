#[cfg(test)]
mod story_graph_tests {
    use raconteur::prelude::{
        Constraint, Entity, Raconteur, StoryCandidate, StoryGraph, StoryNode, StoryWorld,
    };

    #[test]
    fn complex_graph() {
        let mut raconteur = Raconteur::default();
        let mut graph = StoryGraph::new();
        graph.add_alias(
            "rich citizen",
            [
                Constraint::has("city dweller"),
                Constraint::is_in_range_float("wealth", 70.0..100.0),
                Constraint::has("job"),
            ],
        );
        graph.add_alias(
            "poor man",
            [
                Constraint::equals("gender", "male"),
                Constraint::is_in_range_float("wealth", 0.0..5.0),
            ],
        );
        graph.add_alias(
            "little girl",
            [
                Constraint::equals("gender", "female"),
                Constraint::is_in_range_float("age", 5.0..12.0),
            ],
        );

        let start = graph.add(
            StoryNode::new()
                .with_description("rich citizen passes poor man")
                .with_world_constraint(Constraint::equals("location type", "city")),
        );

        let beg_stranger = graph.add(
            StoryNode::new()
                .with_description("poor man begs rich citizen")
                .with_relation_constraints(
                    "poor man",
                    "rich citizen",
                    [Constraint::has_not("knows")],
                ),
        );

        let rich_citizen_donates =
            graph.add(StoryNode::new().with_description("rich citizen donates"));

        let rich_citizen_passes_by =
            graph.add(StoryNode::new().with_description("rich citizen passes by"));

        let beggar_knows_rich_man = graph.add(
            StoryNode::new()
                .with_description("poor man recognizes rich citizen")
                .with_relation_constraints("poor man", "rich citizen", [Constraint::has("knows")]),
        );

        let beggar_talks_about_rich_citizen_daughter = graph.add(
            StoryNode::new()
                .with_description("poor man talks about rich citizen daughter")
                .with_relation_constraints(
                    "rich citizen",
                    "little girl",
                    [Constraint::has("parent")],
                ),
        );

        let beggar_talks_about_his_daughter = graph.add(
            StoryNode::new()
                .with_description("poor man talks about rich citizen daughter")
                .with_relation_constraints(
                    "rich citizen",
                    "little girl",
                    [Constraint::has("parent")],
                ),
        );

        let err_msg = "Failed to connect node";
        graph.set_start_node(start);
        graph.connect(start, beg_stranger).expect(err_msg);
        graph
            .connect(beg_stranger, rich_citizen_donates)
            .expect(err_msg);
        graph
            .connect(beg_stranger, rich_citizen_passes_by)
            .expect(err_msg);
        graph.connect(start, beggar_knows_rich_man).expect(err_msg);
        graph
            .connect(
                beggar_knows_rich_man,
                beggar_talks_about_rich_citizen_daughter,
            )
            .expect(err_msg);
        graph
            .connect(beggar_knows_rich_man, beggar_talks_about_his_daughter)
            .expect(err_msg);
        graph
            .connect_weak(beggar_talks_about_his_daughter, rich_citizen_passes_by)
            .expect(err_msg);
        graph
            .connect_weak(
                beggar_talks_about_rich_citizen_daughter,
                rich_citizen_donates,
            )
            .expect(err_msg);

        raconteur.insert(graph);

        let story_world = StoryWorld::new()
            .with_world_property("location type", "city")
            .with_entities([
                Entity::new(0)
                    .with("city dweller", "")
                    .with("wealth", 80.0)
                    .with("gender", "female")
                    .with("job", "stock broker"),
                Entity::new(1)
                    .with("city dweller", "")
                    .with("wealth", 74.0)
                    .with("gender", "male")
                    .with("job", "real estate agent"),
                Entity::new(2)
                    .with("city dweller", "")
                    .with("wealth", 64.0)
                    .with("gender", "male")
                    .with("job", "manager"),
                Entity::new(3)
                    .with("land dweller", "")
                    .with("wealth", 90.0)
                    .with("gender", "female"),
                Entity::new(4).with("royalty", "").with("wealth", 100.0),
                Entity::new(5).with("gender", "male").with("wealth", 1.0),
                Entity::new(6).with("gender", "male").with("wealth", 3.0),
                Entity::new(7)
                    .with("name", "Emily")
                    .with("gender", "female")
                    .with("age", 6.0),
                Entity::new(8).with("gender", "female").with("age", 11.0),
                Entity::new(9).with("gender", "male").with("age", 10.0),
            ])
            .with_relation(0, 8, "parent", "")
            .with_relation(0, 9, "parent", "")
            .with_relation(1, 8, "parent", "")
            .with_relation(1, 9, "parent", "")
            .with_relation(3, 5, "knows", "")
            .with_relation(0, 4, "knows", "")
            .with_relation(4, 7, "parent", "");

        let result = raconteur.query(&story_world);

        assert!(!result.is_empty());
        let StoryCandidate {
            id,
            alias_candidates,
        } = &result[0];
        assert_eq!(alias_candidates.len(), 8);

        // TODO: problem, if the story world changes a leaf node might not be reachable. What to do in that case? Simply drop the story?
        let story_graph = raconteur.get(*id);
        for alias_map in alias_candidates {
            let mut node_id = story_graph.start();
            while !story_graph
                .next(node_id, &story_world, alias_map)
                .is_empty()
            {
                node_id = story_graph.next(node_id, &story_world, alias_map)[0];
            }
        }
    }
}
