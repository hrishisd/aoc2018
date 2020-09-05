use itertools::iproduct;
use serde_scan::scan;
use std::cmp::max;
use std::collections::HashMap;

type Result<T> = std::result::Result<T, std::boxed::Box<dyn std::error::Error>>;
type Loc = (usize, usize);
type Coord = Loc;

const THRESHOLD_DIST: usize = 10000;

fn main() -> Result<()> {
    let coords: Vec<Coord> = std::fs::read_to_string("input")?
        .lines()
        .map(parse_coordinate)
        .collect::<Result<Vec<Coord>>>()?;

    let (max_x, max_y) = coords.iter().fold((0, 0), |(max_x, max_y), (x, y)| {
        (max(max_x, *x), max(max_y, *y))
    });

    println!("{}", part1(&coords, max_x, max_y)?);
    println!("{}", part2(&coords, max_x, max_y));

    Ok(())
}

fn part2(coords: &[Coord], max_x: usize, max_y: usize) -> usize {
    iproduct!(0..=max_x, 0..=max_y)
        .map(|loc| coords.iter().map(|&coord| manhattan_dist(coord, loc)).sum())
        .filter(|&total_dist: &usize| total_dist < THRESHOLD_DIST)
        .count()
}

fn part1(coords: &[Coord], max_x: usize, max_y: usize) -> Result<usize> {
    let is_boundary = |(x, y)| x == 0 || x == max_x || y == 0 || y == max_y;

    let closest_coord = |loc: Loc| {
        let mut closest: Option<Coord> = Option::None;
        let mut closest_dist = usize::MAX;
        for &coord in coords {
            let dist = manhattan_dist(coord, loc);
            if dist == closest_dist {
                closest = None;
            } else if dist < closest_dist {
                closest_dist = dist;
                closest = Some(coord);
            }
        }
        closest
    };

    let mut coord_to_area_size = HashMap::with_capacity(coords.len());

    for loc in iproduct!(0..=max_x, 0..=max_y) {
        if let Some(coord) = closest_coord(loc) {
            if is_boundary(loc) {
                coord_to_area_size.remove(&coord);
            } else {
                let area = coord_to_area_size.entry(coord).or_insert(0);
                *area += 1;
            }
        }
    }

    coord_to_area_size
        .iter()
        .max_by_key(|(_k, v)| **v)
        .map(|(_k, v)| *v)
        .ok_or_else(|| From::from("empty coord to area map"))
}

fn manhattan_dist((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    ((x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs()) as usize
}

fn parse_coordinate(line: &str) -> Result<Coord> {
    let (x, y): (usize, usize) = scan!("{}, {}" <- line)?;
    Ok((x, y))
}
