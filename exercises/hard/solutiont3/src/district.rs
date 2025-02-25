// src/district.rs
use serde_json::{Value, from_reader};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

// 保留UnionFind实现但标记为pub(crate)
pub(crate) struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub(crate) fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            rank: vec![1; size],
        }
    }

    pub(crate) fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub(crate) fn union(&mut self, x: usize, y: usize) {
        let x_root = self.find(x);
        let y_root = self.find(y);
        if x_root == y_root {
            return;
        }
        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else {
            self.parent[y_root] = x_root;
            if self.rank[x_root] == self.rank[y_root] {
                self.rank[x_root] += 1;
            }
        }
    }
}

// 确保函数被导出
pub fn process_districts() {
    let file = File::open("district.json").expect("Failed to open file");
    let reader = BufReader::new(file);
    let json_data: Value = from_reader(reader).expect("Failed to parse JSON");

    let batches = json_data.as_object().expect("Invalid JSON format");
    let mut sorted_batches: Vec<_> = batches.iter().collect();
    sorted_batches.sort_by_key(|(k, _)| k.parse::<i32>().unwrap());

    let mut results = Vec::new();
    for (_, batch_value) in sorted_batches {
        let cities = batch_value.as_object().expect("Batch data should be object");

        let mut merged = HashMap::new();
        for (city, neighbors) in cities {
            let entry = merged.entry(city.as_str()).or_insert(Vec::new());
            for n in neighbors.as_array().unwrap() {
                entry.push(n.as_str().unwrap());
            }
        }

        let all_cities: HashSet<_> = merged.iter()
            .flat_map(|(&k, v)| std::iter::once(k).chain(v.iter().copied()))
            .collect();

        let mut cities: Vec<_> = all_cities.into_iter().collect();
        cities.sort_unstable();
        let city_indices: HashMap<_, _> = cities.iter()
            .enumerate()
            .map(|(i, &city)| (city, i))
            .collect();

        let mut uf = UnionFind::new(cities.len());

        for (city, neighbors) in &merged {
            let &x = city_indices.get(city).unwrap();
            for &neighbor in neighbors {
                let &y = city_indices.get(neighbor).unwrap();
                uf.union(x, y);
            }
        }

        let roots: HashSet<_> = (0..cities.len()).map(|i| uf.find(i)).collect();
        results.push(roots.len().to_string());
    }

    println!("{}", results.join(","));
}