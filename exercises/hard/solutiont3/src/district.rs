use crate::json;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::str::FromStr;

struct Graph {
    adj_list: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            adj_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, u: String, v: String) {
        self.adj_list
            .entry(u.clone())
            .or_insert(vec![])
            .push(v.clone());
        self.adj_list.entry(v).or_insert(vec![]).push(u); // Undirected graph
    }

    fn count_closed_loops(&self) -> usize {
        let mut visited: HashSet<String> = HashSet::new();
        let mut cycle_count = 0;

        for node in self.adj_list.keys() {
            if !visited.contains(node) {
                if self.dfs(node.clone(), None, &mut visited, &mut HashSet::new()) {
                    cycle_count += 1;
                }
            }
        }

        cycle_count
    }

    fn dfs(
        &self,
        node: String,
        parent: Option<String>,
        visited: &mut HashSet<String>,
        stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(node.clone());
        stack.insert(node.clone());

        if let Some(neighbors) = self.adj_list.get(&node.clone()) {
            for neighbor in neighbors {
                if Some(neighbor) == parent.as_ref() {
                    continue; // Skip the edge leading back to the parent
                }
                if stack.contains(neighbor)
                    || (!visited.contains(neighbor)
                        && self.dfs(neighbor.clone(), Some(node.clone()), visited, stack))
                {
                    return true; // Cycle detected
                }
            }
        }

        stack.remove(&node);
        false
    }
}

pub fn count_provinces() -> String {
    let json = read_to_string("district.json").unwrap();
    let json = json::JsonObject::parse(json.as_str()).unwrap();
    let mut provinces = HashMap::new();

    json.object().unwrap().iter().for_each(|(main_key, v)| {
        let mut graph = Graph::new();

        v.object().unwrap().iter().for_each(|(key, array)| {
            array.array().unwrap().iter().for_each(|p| {
                graph.add_edge(key.to_string(), p.string().unwrap().clone());
            })
        });

        provinces.insert(
            u8::from_str(main_key.as_str()).unwrap(),
            graph.count_closed_loops() as u32,
        );
    });

    let mut result = String::new();
    provinces
        .iter_mut()
        .filter(|(_, count)| **count == 0)
        .for_each(|(_, count)| *count = 1u32);
    let mut sorted_vec: Vec<_> = provinces.iter().map(|(&k, &v)| (k, v)).collect();
    sorted_vec.sort_by_key(|&(k, _)| k);
    sorted_vec.iter().for_each(|(_, v)| {
        result.push_str(&v.to_string());
        result.push(',');
    });
    result.pop();
    result
}
