mod input;

use std::collections::HashMap;

type Tree = char;
type Trees = HashMap<String, Tree>;

fn trees(input: &str) -> Trees {
    let rows = input.split('\n');

    let mut trees: Trees = HashMap::new();
    for (y, row) in rows.enumerate() {
        for (x, tree) in row.chars().enumerate() {
            let coord = format!("{}-{}", x, y);
            trees.insert(coord, tree);
        }
    }

    return trees;
}

fn to_ints(coord: &String) -> (u32, u32) {
    let (x, y) = coord.split_once('-').unwrap();
    let x = x.parse::<u32>().unwrap();
    let y = y.parse::<u32>().unwrap();
    (x, y)
}

fn is_taller_than(
    trees: &Trees,
    coords: impl Iterator<Item = (u32, u32)>,
    tree: &Tree
) -> bool {
    coords
        .map(|(x, y)| format!("{}-{}", x, y))
        .map(|coord| trees.get(&coord).unwrap().clone())
        .all(|other_tree| other_tree < *tree)
}

fn max_x(trees: &Trees) -> u32 {
    trees
        .keys()
        .map(|coord| to_ints(coord))
        .map(|(x, _)| x)
        .max()
        .unwrap()
}

fn max_y(trees: &Trees) -> u32 {
    trees
        .keys()
        .map(|coord| to_ints(coord))
        .map(|(_, y)| y)
        .max()
        .unwrap()
}

fn is_visible_from_left(trees: &Trees, coord: &String, tree: &Tree) -> bool {
    let (x, y) = to_ints(coord);
    let range = (0..x).map(|x| (x, y));
    let result = is_taller_than(trees, range, tree);
    result
}

fn is_visible_from_top(trees: &Trees, coord: &String, tree: &Tree) -> bool {
    let (x, y) = to_ints(coord);
    let range = (0..y).map(|y| (x, y));
    let result = is_taller_than(trees, range, tree);
    result
}

fn is_visible_from_right(trees: &Trees, coord: &String, tree: &Tree, max_x: &u32) -> bool {
    let (x, y) = to_ints(coord);
    let range = ((x+1)..(max_x+1)).map(|x| (x, y));
    let result = is_taller_than(trees, range, tree);
    result
}

fn is_visible_from_bottom(trees: &Trees, coord: &String, tree: &Tree, max_y: &u32) -> bool {
    let (x, y) = to_ints(coord);
    let range = ((y+1)..(max_y+1)).map(|y| (x, y));
    is_taller_than(trees, range, tree)
}

fn is_visible_from_edge(
    trees: &Trees,
    coord: &String,
    tree: &Tree,
    max_x: &u32,
    max_y: &u32
) -> bool {
    is_visible_from_left(trees, coord, tree)
    || is_visible_from_top(trees, coord, tree)
    || is_visible_from_right(trees, coord, tree, max_x)
    || is_visible_from_bottom(trees, coord, tree, max_y)
}

fn part1(input: &str) -> usize {
    let trees = trees(input);

    let max_x = max_x(&trees);
    let max_y = max_y(&trees);

    trees
        .iter()
        .filter(|(coord, tree)| is_visible_from_edge(&trees, coord, tree, &max_x, &max_y))
        .count()
}

pub fn test() {
    assert_eq!(part1(input::TEST), 21);
    assert_eq!(part1(input::REAL), 1713);
}
