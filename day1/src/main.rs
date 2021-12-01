use itertools::Itertools;

pub fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt")
        .split('\n')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<_>, _>>()?;

    println!("Solution 1: {:?}", solution_1(&input));
    println!("Solution 2: {:?}", solution_2(&input));
    Ok(())
}

pub fn solution_1(input: &[i64]) -> usize {
    input
        .iter()
        .tuple_windows()
        .filter(|(h1, h2)| h2 > h1)
        .count()
}

pub fn solution_2(input: &[i64]) -> usize {
    input
        .iter()
        .tuple_windows()
        .map(|(h1, h2, h3)| h1 + h2 + h3)
        .tuple_windows()
        .filter(|(h1, h2)| h2 > h1)
        .count()
}
