use itertools::Itertools;

fn main() {
    let words = include_str!("../words.txt").lines().collect::<Vec<_>>();
    
    let input = std::env::args().nth(1).unwrap();
    let possibilities = input.chars().powerset().map(|x| {let len = x.len(); x.into_iter().permutations(len)}).flatten().map(|cs| cs.into_iter().collect::<String>());

    for possibility in possibilities {
        if words.contains(&possibility.as_str()) {
            println!("{possibility}");
        }
    }
}
