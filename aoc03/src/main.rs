use itertools::Itertools;
use serde_scan::scan;
use std::collections::{HashMap, HashSet};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Claim {
    id: usize,
    left_offset: usize,
    top_offset: usize,
    width: usize,
    height: usize,
}
fn main() -> Result<()> {
    let input = std::fs::read_to_string("input")?;
    let claims = parse_claims(&input)?;
    println!(
        "{} sq inches have at least 2 overlapping claims",
        sq_inches_claimed_twice(&claims)
    );
    println!(
        "{} does not overlap with any other claims",
        find_nonoverlapping_claim(&claims)
            .ok_or("didn't find unique claim")?
            .id
    );
    Ok(())
}

fn find_nonoverlapping_claim(claims: &[Claim]) -> Option<&Claim> {
    let mut nonoverlapping: HashSet<&Claim> = HashSet::new();
    for claim in claims {
        nonoverlapping.insert(claim);
    }

    for claim in claims {
        for inner_claim in claims {
            if claim != inner_claim && claims_overlap(claim, inner_claim) {
                nonoverlapping.remove(claim);
                nonoverlapping.remove(inner_claim);
            }
        }
    }
    nonoverlapping.into_iter().next()
}

fn claims_overlap(claim1: &Claim, claim2: &Claim) -> bool {
    overlaps(
        (claim1.left_offset, claim1.left_offset + claim1.width),
        (claim2.left_offset, claim2.left_offset + claim2.width),
    ) && overlaps(
        (claim1.top_offset, claim1.top_offset + claim1.height),
        (claim2.top_offset, claim2.top_offset + claim2.height),
    )
}

fn overlaps(range1: (usize, usize), range2: (usize, usize)) -> bool {
    let (l, r) = if range1.0 <= range2.0 {
        (range1, range2)
    } else {
        (range2, range1)
    };
    r.0 < l.1
}

fn sq_inches_claimed_twice(claims: &[Claim]) -> usize {
    let mut coord_to_num_claims: HashMap<(usize, usize), usize> = HashMap::new();
    for claim in claims {
        for coord in coords_in_claim(claim) {
            let entry = coord_to_num_claims.entry(coord).or_insert(0);
            *entry += 1;
        }
    }
    coord_to_num_claims
        .values()
        .filter(|count| **count >= 2usize)
        .count()
}

fn coords_in_claim(claim: &Claim) -> impl Iterator<Item = (usize, usize)> {
    let x_coords = claim.left_offset..(claim.left_offset + claim.width);
    let y_coords = claim.top_offset..(claim.top_offset + claim.height);
    x_coords.cartesian_product(y_coords)
}

fn parse_claims(input: &str) -> Result<Vec<Claim>> {
    input.lines().map(parse_claim).collect()
}

fn parse_claim(line: &str) -> Result<Claim> {
    let (id, offset_from_left, offset_from_top, width, height): (
        usize,
        usize,
        usize,
        usize,
        usize,
    ) = scan!("#{} @ {},{}: {}x{}" <- line)?;

    Ok(Claim {
        id,
        left_offset: offset_from_left,
        top_offset: offset_from_top,
        width,
        height,
    })
}
