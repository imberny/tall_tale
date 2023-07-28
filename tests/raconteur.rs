#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use raconteur::prelude::*;

    const GUY_ID: i64 = 0;
    const GIRL_ID: i64 = 1;

    fn query() -> Query {
        let character1 = Entity::builder(GUY_ID)
            .with("name", "Bertrand")
            .with("age", 30)
            .build();
        let character2 = Entity::builder(GIRL_ID)
            .with("name", "Juliette")
            .with("age", 32)
            .build();

        let relationships = RelationMap::from([(
            (GUY_ID, GIRL_ID),
            PropertyMap::from([("opinion".to_string(), 2.into())]),
        )]);

        Query {
            entities: vec![character1, character2],
            entity_relations: relationships,
            world_state: PropertyMap::default(),
        }
    }

    fn guy_no_like_girl() -> Raconteur {
        let mut raconteur = Raconteur::new();
        raconteur.push(
            StoryNode::builder()
                .with_description("low_opinion")
                .with_alias("guy", vec![])
                .with_alias("girl", vec![])
                .with_relation(
                    "guy",
                    "girl",
                    PropertyConstraint::IsInRange(
                        "opinion".to_string(),
                        std::ops::Range { start: 0, end: 1 },
                    ),
                )
                .build(),
        );

        raconteur
    }

    fn guy_like_girl() -> Raconteur {
        let mut raconteur = Raconteur::new();
        raconteur.push(
            StoryNode::builder()
                .with_description("guy_like_girl")
                .with_alias("guy", vec![])
                .with_alias("girl", vec![])
                .with_relation(
                    "guy",
                    "girl",
                    PropertyConstraint::IsInRange(
                        "opinion".to_string(),
                        std::ops::Range { start: 1, end: 4 },
                    ),
                )
                .build(),
        );
        raconteur
    }

    #[test]
    fn no_match() {
        let raconteur = guy_no_like_girl();
        let beat_candidates = raconteur.query(&query());
        assert!(beat_candidates.is_empty());
    }

    #[test]
    fn a_match() {
        let raconteur: Raconteur = guy_like_girl();
        let beat_candidates = raconteur.query(&query());
        let beat = Rc::clone(&beat_candidates[0].0);
        assert_eq!(beat.description, "guy_like_girl");
    }
}
