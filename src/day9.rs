
type ValueType = i64;

fn read_numbers(s: &str) -> Result<Vec<ValueType>, String> {
  Ok(s.split_whitespace()
      .map(|w| w.parse::<ValueType>().map_err(|_| format!("Can't parse number {w}")))
      .collect::<Result<Vec<ValueType>, String>>()?)
}

pub fn generator(input: &str) -> Vec<Vec<ValueType>> {
  input.lines().map(read_numbers).collect::<Result<Vec<Vec<ValueType>>,String>>()
      .unwrap() // panics on error
}

fn process_series(series: &[ValueType]) -> ValueType {
  if series.iter().all(| val| *val == 0) {
    0
  } else {
    let next = series[1..].iter().enumerate()
        .map(|(i, val) | val - series[i]).collect::<Vec<ValueType>>();
    process_series(&next) + series.last().unwrap()
  }
}

pub fn part1(input: &[Vec<ValueType>]) -> ValueType {
  input.iter().map(|v| process_series(v)).sum()
}

pub fn part2(input: &[Vec<ValueType>]) -> ValueType {
  0
}

#[cfg(test)]
mod tests {
  use crate::day9::{generator, part1, part2};

  const INPUT: &str =
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

  #[test]
  fn test_part1() {
    assert_eq!(114, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(0, part2(&generator(INPUT)));
  }
}
