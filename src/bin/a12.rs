use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Graph, Undirected};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("input/12.txt")?;
    let edges: Vec<(&str, &str)> = file
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .collect();
    let mut nodes = HashMap::<&str, NodeIndex>::new();
    let mut graph = Graph::<&str, (), Undirected>::new_undirected();
    for (a, b) in &edges {
        let a_node = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
        let b_node = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
        graph.add_edge(a_node, b_node, ());
    }

    let mut queue = vec![vec![nodes["start"]]];
    let mut total = 0;
    while let Some(path) = queue.pop() {
        let last = *path.last().unwrap();
        if graph[last] == "end" {
            total += 1;
            continue;
        }
        for neighbor in graph.neighbors(last) {
            if !path.contains(&neighbor) || graph[neighbor].chars().all(char::is_uppercase) {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                queue.push(new_path);
            }
        }
    }
    println!("1: {}", total);

    let mut queue = vec![(vec![nodes["start"]], false)];
    let mut total = 0;
    while let Some((path, double_visited)) = queue.pop() {
        let last = *path.last().unwrap();
        if graph[last] == "end" {
            total += 1;
            continue;
        }
        for neighbor in graph.neighbors(last) {
            if !path.contains(&neighbor) || graph[neighbor].chars().all(char::is_uppercase) {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                queue.push((new_path, double_visited));
            }
            if path.contains(&neighbor)
                && graph[neighbor].chars().all(char::is_lowercase)
                && !double_visited
                && !["start", "end"].contains(&graph[neighbor])
            {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                queue.push((new_path, true));
            }
        }
    }
    println!("2: {}", total);
    Ok(())
}
