use std::char;
fn main() {
    let input = 327901;
    println!("{}", part1(input));
    println!("{}", part2(&[3, 2, 7, 9, 0, 1]));
}

fn part1(input: usize) -> String {
    let recipes = generate_recipes(input + 10);
    let last_ten = &recipes[input..input + 10];
    last_ten
        .iter()
        .map(|&x| char::from_digit(x as u32, 10).unwrap())
        .collect()
}

fn generate_recipes(num_recipes: usize) -> Vec<usize> {
    // allocate enough space for all the recipes up front
    println!("started allocating");
    let mut recipes = Vec::with_capacity(num_recipes + 20);
    println!("finished allocating");
    recipes.push(3);
    recipes.push(7);

    // These two indexes into the recipes vec represent the two elves
    let mut first = 0;
    let mut second = 1;

    while recipes.len() < num_recipes {
        let recipe_sum = recipes[first] + recipes[second];
        if recipe_sum > 9 {
            recipes.push(recipe_sum / 10);
        }
        recipes.push(recipe_sum % 10);
        first = (first + recipes[first] + 1) % recipes.len();
        second = (second + recipes[second] + 1) % recipes.len();
    }
    recipes
}

fn part2(needle: &[usize]) -> usize {
    // allocate enough space for all the recipes up front
    let mut recipes: Vec<usize> = Vec::new();
    recipes.push(3);
    recipes.push(7);

    // These two indexes into the recipes vec represent the two elves
    let mut first = 0;
    let mut second = 1;

    loop {
        let recipe_sum = recipes[first] + recipes[second];
        if recipe_sum > 9 {
            recipes.push(recipe_sum / 10);
        }
        recipes.push(recipe_sum % 10);

        for offset in 0..=1 {
            if recipes.len() > needle.len() + offset {
                let num_skip = recipes.len() - needle.len() - offset;
                let tail = &recipes[num_skip..num_skip + needle.len()];
                if tail == needle {
                    println!("tail: {:?}", tail);
                    println!("needle: {:?}", needle);
                    return num_skip;
                }
            }
        }
        if (recipes.len() % 10000000 == 0) {
            println!("{}", recipes.len());
        }

        first = (first + recipes[first] + 1) % recipes.len();
        second = (second + recipes[second] + 1) % recipes.len();
    }
}

#[test]
fn test1() {
    assert_eq!(part1(5), "0124515891");
    assert_eq!(part1(18), "9251071085");
    assert_eq!(part1(2018), "5941429882");
}

#[test]
fn test2() {
    assert_eq!(part2(&[0, 1, 2, 4, 5]), 5);
    assert_eq!(part2(&[5, 1, 5, 8, 9]), 9);
    assert_eq!(part2(&[9, 2, 5, 1, 0]), 18);
    assert_eq!(part2(&[5, 9, 4, 1, 4]), 2018);
}
