use std::collections::{HashMap,HashSet};


#[derive(Debug)]
struct Graph(HashMap<String, HashSet<String>>);

impl Graph {
    fn new() -> Graph {
        Graph(HashMap::new())
    }

    fn connect(&mut self, a: &str, b: &str) {
        self.0.entry(a.to_string())
            .and_modify(|s| { s.insert(b.to_string()); })
            .or_insert(HashSet::from([b.to_string()]));

        self.0.entry(b.to_string())
            .and_modify(|s| { s.insert(a.to_string()); })
            .or_insert(HashSet::from([a.to_string()]));
    }

    fn collect(&self) -> Vec<String> {
        let mut visited = HashSet::<String>::new();
        fn dfs(graph: &Graph, visited: &mut HashSet<String>, node: &String) {
            if visited.contains(node) {
                return;
            }
            visited.insert(node.clone());
            let children = match graph.0.get(node) {
                Some(v) => v,
                None => return,
            };
            children.iter().for_each(|n| dfs(graph, visited, n));
        }

        self.0.keys().for_each(|k| dfs(&self, &mut visited, k));
        let mut v = visited.into_iter().collect::<Vec<String>>();
        v.sort();
        v
    }
}

#[test]
fn graph_works() {
    let data = vec![
        ("A","B"),
        ("A","C"),
        ("C","D"),
        ("E","F"),
    ];
    let mut g = Graph::new();
    data.iter().for_each(|d| g.connect(d.0, d.1));
    assert_eq!(g.collect(), vec!["A", "B", "C", "D", "E", "F"])
}