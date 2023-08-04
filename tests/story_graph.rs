#[cfg(test)]
mod tests {
    use raconteur::prelude::{StoryGraph, StoryNode};

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
}
