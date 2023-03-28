pub mod graph {
    use std::collections::HashMap;

    use self::graph_items::{edge::Edge, node::Node};

    type Attr<'a> = (&'a str, &'a str);

    fn attrs_to_map(attrs: &[Attr]) -> HashMap<String, String> {
        let mut map = HashMap::with_capacity(attrs.len());
        attrs.iter().for_each(|&(k, v)| {
            map.insert(k.to_string(), v.to_string());
        });
        map
    }

    #[derive(Debug)]
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Self {
                nodes: vec![],
                edges: vec![],
                attrs: HashMap::new(),
            }
        }

        pub fn with_nodes(self, nodes: &[Node]) -> Self {
            Self {
                nodes: nodes.to_vec(),
                ..self
            }
        }

        pub fn with_edges(self, edges: &[Edge]) -> Self {
            Self {
                edges: edges.to_vec(),
                ..self
            }
        }

        pub fn with_attrs(self, attrs: &[Attr]) -> Self {
            Self {
                attrs: attrs_to_map(attrs),
                ..self
            }
        }

        pub fn node(&self, key: &str) -> Option<&Node> {
            self.nodes.iter().find(|&node| node.value() == key)
        }
    }

    pub mod graph_items {
        pub mod edge {
            use crate::graph::{attrs_to_map, Attr};
            use std::collections::HashMap;

            #[derive(Debug, PartialEq, Clone)]
            pub struct Edge {
                start: String,
                end: String,
                attrs: HashMap<String, String>,
            }

            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Self {
                        start: a.to_string(),
                        end: b.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[Attr]) -> Self {
                    Self {
                        attrs: attrs_to_map(attrs),
                        ..self
                    }
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|str| str.as_str())
                }
            }
        }

        pub mod node {
            use std::collections::HashMap;

            use crate::graph::{attrs_to_map, Attr};
            #[derive(Debug, PartialEq, Clone)]
            pub struct Node {
                value: String,
                attrs: HashMap<String, String>,
            }

            impl Node {
                pub fn new(value: &str) -> Self {
                    Self {
                        value: value.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(self, attrs: &[Attr]) -> Self {
                    Self {
                        attrs: attrs_to_map(attrs),
                        ..self
                    }
                }

                pub fn attr(&self, attr: &str) -> Option<&str> {
                    self.attrs.get(attr).map(|str| str.as_str())
                }

                pub fn value(&self) -> &str {
                    &self.value.as_str()
                }
            }
        }
    }
}
