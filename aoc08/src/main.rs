// use itertools::Itertools;
type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;
#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata_entries: Vec<usize>,
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input")?;
    let root = build_tree(&input)?;
    println!("part 1: {}", part1(&root));
    println!("part 2: {}", part2(&root));
    Ok(())
}

fn part1(root: &Node) -> usize {
    sum_metadata_rec(root)
}

fn sum_metadata_rec(node: &Node) -> usize {
    node.metadata_entries.iter().sum::<usize>()
        + node.children.iter().map(sum_metadata_rec).sum::<usize>()
}

fn part2(root: &Node) -> usize {
    calculate_value_rec(&root)
}

fn calculate_value_rec(node: &Node) -> usize {
    if node.children.len() == 0 {
        node.metadata_entries.iter().sum()
    } else {
        node.metadata_entries
            .iter()
            .map(|&i| {
                node.children
                    .get(i - 1)
                    .map(calculate_value_rec)
                    .unwrap_or(0)
            })
            .sum()
    }
}

fn build_tree(input: &str) -> Result<Node> {
    build_tree_rec(&mut input.split_ascii_whitespace().map(|s| s.parse::<usize>()))
}

fn build_tree_rec<I>(vals: &mut I) -> Result<Node>
where
    I: Iterator<Item = std::result::Result<usize, std::num::ParseIntError>>,
{
    let num_children = vals.next().ok_or("incomplete input")??;
    let num_metadata_entries = vals.next().ok_or("incomplete input")??;
    let mut children = Vec::with_capacity(num_children);
    let mut metadata_entries = Vec::with_capacity(num_metadata_entries);

    for _ in 0..num_children {
        children.push(build_tree_rec(vals)?);
    }

    for _ in 0..num_metadata_entries {
        metadata_entries.push(vals.next().ok_or("incomplete input")??);
    }
    // let num_metadata_entries = vals.nex
    Ok(Node {
        children,
        metadata_entries,
    })
}
