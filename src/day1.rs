use std::cmp;

/// One integer per a line with groups divided by blank lines
///   1
///   2
///
///   3
/// = vec!{vec!{1, 2}, vec!{3}}
pub fn generator(input: &str) -> Vec<Vec<i32>> {
  input.split("\n\n")
    .map(|section|
       section.lines().filter_map(|line| line.parse().ok()).collect())
    .collect()
}

/// Sum each group and find the maximum
pub fn part1(input: &[Vec<i32>]) -> i32 {
  input.iter()
    .map(|v| v.iter().fold(0, |a, &b| a + b))
    .reduce(|a, b| cmp::max(a,b)).unwrap()
}

/// Add the three largest groups
pub fn part2(input: &[Vec<i32>]) -> i32 {
  let mut calories: Vec<i32> = input.iter()
    .map(|v| v.iter().fold(0, |a, &b| a + b)).collect();
  calories.sort_unstable_by(|a, b| b.cmp(a));
  calories.iter().take(3).fold(0, |a, &b| a + b)
}

#[cfg(test)]
mod tests {
  use crate::day1::{generator, part1, part2};

  #[test]
  fn parsing_test() {
    let result= generator("1\n2\n\n3\n4\n5");
    assert_eq!(vec!{vec!{1, 2}, vec!{3, 4, 5}}, result);
  }

  const INPUT: &str = "1000\n2000\n3000\n\n\
                       4000\n\n\
                       5000\n6000\n\n\
                       7000\n8000\n9000\n\n\
                       10000";

  #[test]
  fn test_part1() {
    assert_eq!(24000, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(45000, part2(&generator(INPUT)));
  }
}
