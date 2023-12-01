pub fn generator(input: &str) -> Vec<String> {
  input.lines().map(|l| l.to_string()).collect()
}

/// Add each line as first and last digit
pub fn part1(input: &[String]) -> i32 {
  input.iter()
    .map(|v| {
      let first = v.chars().find(|c| c.is_ascii_digit()).unwrap();
      let second = v.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
      (first as i32 - '0' as i32) * 10 + (second as i32 - '0' as i32)})
    .sum()
}

fn digit(str: &str) -> Option<i32> {
  match str.chars().next() {
    Some('o') => if str.starts_with("one") { return Some(1) },
    Some('t') => if str.starts_with("two") { return Some(2) }
      else if str.starts_with("three") { return Some(3) },
    Some('f') => if str.starts_with("four") { return Some(4) }
      else if str.starts_with("five") { return Some(5) },
    Some('s') => if str.starts_with("six") { return Some(6) }
      else if str.starts_with("seven") { return Some(7) },
    Some('e') => if str.starts_with("eight") { return Some(8) },
    Some('n') => if str.starts_with("nine") { return Some(9) },
    Some(ch) => if ch.is_ascii_digit() { return Some(ch as i32 - '0' as i32)},
    _ => {},
  }
  None
}

fn first_digit(s: &str) -> i32 {
  for i in 0..s.len() {
    if let Some(d) = digit(&s[i..]) {
      return d
    }
  }
  0
}

fn last_digit(s: &str) -> i32 {
  for i in (0..s.len()).rev() {
    if let Some(d) = digit(&s[i..]) {
      return d
    }
  }
  0
}

/// Include the word replacements for the digits.
pub fn part2(input: &[String]) -> i32 {
  input.iter().map(|l| {
      first_digit(l) * 10 + last_digit(l)})
    .sum()
}

#[cfg(test)]
mod tests {
  use crate::day1::{generator, part1, part2};

  const INPUT: &str =
"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

  #[test]
  fn test_part1() {
    assert_eq!(142, part1(&generator(INPUT)));
  }

  const INPUT2: &str =
"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

  #[test]
  fn test_part2() {
    assert_eq!(142, part2(&generator(INPUT)));
    assert_eq!(281, part2(&generator(INPUT2)));
    assert_eq!(21, part2(&generator("twone")));
  }
}
