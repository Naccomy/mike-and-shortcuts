use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::stdin,
};

fn main() {
    let mut input_lines = input_lines();

    let n = input_lines.next().unwrap().parse::<u32>().unwrap();
    let shortcut_line = input_lines.next().unwrap();
    let shortcuts = shortcut_line
        .split_ascii_whitespace()
        .map(|elt| elt.parse::<u32>().unwrap());

    let solution = resolve(n, shortcuts);
    print_solution(solution);
}

fn resolve<I>(n: u32, shortcuts: I) -> Vec<u32>
where
    I: Iterator<Item = u32>,
{
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    // Build base graph
    for intersection in 2..=n {
        graph
            .entry(intersection - 1)
            .or_default()
            .push(intersection);

        graph
            .entry(intersection)
            .or_default()
            .push(intersection - 1);
    }

    // Add shortcuts
    for (start, end) in (1..).zip(shortcuts) {
        graph.entry(start).or_default().push(end);
    }

    let mut result = vec![];
    for i in 1..=n {
        let val = bfs(&graph, i, HashSet::new());
        result.push(val);
    }

    result
}

fn bfs(graph: &HashMap<u32, Vec<u32>>, target: u32, mut visited: HashSet<u32>) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back(Some(1));
    queue.push_back(None);
    let mut distance = 0;

    while let Some(node) = queue.pop_front() {
        let Some(node) = node else {
            // We finished processing neighbors
            distance += 1;
            if !queue.is_empty() {
                queue.push_back(None);
            }
            continue;
        };

        // If already visited, skip
        if !visited.insert(node) {
            continue;
        }

        if node == target {
            return distance;
        }

        for neighbor in graph.get(&node).unwrap() {
            queue.push_back(Some(*neighbor));
        }
    }

    unreachable!("No path found...")
}

fn print_solution(solution: Vec<u32>) {
    let s = solution
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    println!("{s}")
}

fn input_lines() -> impl Iterator<Item = String> {
    stdin().lines().map(|l| l.unwrap())
}
