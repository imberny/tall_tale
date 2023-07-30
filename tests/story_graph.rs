#[cfg(test)]
mod tests {
    use raconteur::prelude::{Constraint, StoryGraph, StoryNode};

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

    // TODO: maybe explicit inheritance isn't needed.. see StoryNode
    #[test]
    fn graph_child_inheritance() {
        let mut graph = StoryGraph::new();
        let parent_idx = graph.add(
            StoryNode::new()
                .with_alias_constraints("me", [Constraint::has("hello")])
                .with_alias_constraints(
                    "other",
                    [Constraint::equals("test property", "test value")],
                )
                .with_relation_constraints("me", "other", [Constraint::has("test")])
                .with_world_constraint(Constraint::is_in_range_float("test range", 0.0..100.0)),
        );
        let child_idx = graph.add(StoryNode::new());

        let _ = graph.connect(parent_idx, child_idx);

        {
            let parent = graph.get(parent_idx);
            let child = graph.get(child_idx);

            assert_eq!(parent.constraints, child.inherited_constraints);
        }

        // weak connections don't inherit
        let weak_child_idx = graph.add(StoryNode::new());
        let _ = graph.connect_weak(parent_idx, weak_child_idx);

        {
            let parent = graph.get(parent_idx);
            let weak_child = graph.get(weak_child_idx);
            assert_ne!(parent.constraints, weak_child.inherited_constraints);
        }

        let child_with_constraints_idx = graph.add(
            StoryNode::new()
                .with_alias_constraints("in child 1", [Constraint::has("child constraint")])
                .with_alias_constraints("in child 2", [])
                .with_relation_constraints(
                    "in child 1",
                    "in child 2",
                    [Constraint::has("in child")],
                )
                .with_world_constraint(Constraint::has("in child world constraint")),
        );

        let _ = graph.connect(parent_idx, child_with_constraints_idx);
        let _ = graph.connect_weak(child_with_constraints_idx, parent_idx);

        {
            // cycle with weak
            let parent = graph.get(parent_idx);
            let child = graph.get(child_with_constraints_idx);

            // child should have inherited from parent, but should still have unique constraints
            assert_eq!(parent.constraints, child.inherited_constraints);
            assert_ne!(parent.constraints, child.constraints);
        }
    }
}
