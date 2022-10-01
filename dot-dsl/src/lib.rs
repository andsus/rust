use std::collections::HashMap;

macro_rules! impl_attrs {
    () => {
        pub fn get_attr(&self, key: &str) -> Option<&str> {
            self.attrs.get(key).map(|s| s.as_str())
        }

        pub fn with_attrs(mut self, attrs: &[(&str,&str)]) -> Self {
            self.attrs = attrs
            .iter()
            .map(|(a,b)| (a.to_string(), b.to_string()))
            .collect();
            self
        }
        
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub attrs: HashMap<String, String>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nodes(mut self, nodes: &Vec<Node>) -> Self {
        self.nodes = nodes.to_vec();
        self
    }

    pub fn with_edges(mut self, edges: &Vec<Edge>) -> Self {
        self.edges = edges.to_vec();
        self
    }

    pub fn get_node(&self, name: &str ) -> Option<&Node> {
        self.nodes.iter().find(|&n| n.name == name)
    }
    impl_attrs!();
}

#[derive(Default, Debug, Clone, PartialEq,Eq)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub attrs: HashMap<String, String>,
}
impl Edge {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_owned(),
            to: to.to_owned(),
            ..Self::default()
        }
    }

    impl_attrs!();
}

#[derive(Default, Debug, Clone, PartialEq,Eq)]
pub struct Node {
    pub name: String,
    pub attrs: HashMap<String, String>,
}
impl Node {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Self::default()
        }
    }

    impl_attrs!();
}

pub mod graph {
    pub use super::Graph;
    pub mod graph_items {
        pub mod edge {
            pub use super::super::super::Edge;
        }
        pub mod node {
            pub use super::super::super::Node;
        }
    }
}