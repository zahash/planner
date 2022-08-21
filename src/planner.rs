use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

pub trait Graph<T> {
    type Output: IntoIterator<Item = T>;
    fn adjacent(&self, node: &T) -> Self::Output;
}

#[derive(Clone, Debug)]
pub struct Path<T: Eq + Hash + Clone> {
    node: T,
    steps: usize,
    parent: Box<Option<Path<T>>>,
}

pub fn bft<T: Eq + Hash + Clone>(start: T, graph: impl Graph<T>) -> Vec<Path<T>> {
    let mut frontier = VecDeque::from([Path {
        node: start,
        steps: 0,
        parent: Box::new(Option::None),
    }]);

    let mut seen: HashSet<T> = HashSet::new();

    let mut paths: Vec<Path<T>> = vec![];

    while !frontier.is_empty() {
        let path = frontier.pop_front().unwrap();
        if seen.contains(&path.node) {
            continue;
        }

        paths.push(path.clone());
        seen.insert(path.node.clone());

        for adj_node in graph.adjacent(&path.node) {
            frontier.push_back(Path {
                node: adj_node.clone(),
                steps: path.steps + 1,
                parent: Box::new(Option::Some(path.clone())),
            })
        }
    }

    paths
}

pub fn bfs<T: Eq + Hash + Clone>(start: T, goal: T, graph: impl Graph<T>) -> Option<Path<T>> {
    for path in bft(start, graph) {
        if path.node == goal {
            return Option::Some(path);
        }
    }
    Option::None
}
