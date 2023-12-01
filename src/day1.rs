use lazy_static::lazy_static;
use regex::Regex;

pub fn generator(input: &str) -> Vec<String> {
  input.lines().map(|l| l.to_string()).collect()
}

/// Add each line as first and last digit
pub fn part1(input: &[String]) -> i32 {
  input.iter()
    .map(|v| {
      let first = v.chars().find(|c| c.is_ascii_digit()).unwrap();
      let second = v.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
      format!("{first}{second}").parse::<i32>().unwrap()})
    .sum()
}

// a regex for matching the patterns we are looking for
lazy_static! {
  static ref DIGIT_REGEX: Regex =
    Regex::new(r"([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
}

/// Translate a 'digit' to the corresponding number.
fn translate_digit(s: &str) -> i32 {
  match s {
    "0" => 0,
    "1" | "one" => 1,
    "2" | "two" => 2,
    "3" | "three" => 3,
    "4" | "four" => 4,
    "5" | "five" => 5,
    "6" | "six" => 6,
    "7" | "seven" => 7,
    "8" | "eight" => 8,
    "9" | "nine" => 9,
    _ => panic!("Not a digit!"),
  }
}

/// Include the word replacements for the digits.
pub fn part2(input: &[String]) -> i32 {
  input.iter().map(|l| {
      let mut iter = DIGIT_REGEX.find_iter(l);
      let first = iter.next().unwrap();
      let first_digit = translate_digit(first.as_str());
      let mut second = iter.last();
      // if there isn't a second match, we should reuse the first
      if second.is_none() {
        second = Some(first);
      }
      // We have to make sure there isn't another match that was hidden by
      // the one we found. For example, 'twone' the regex will just find
      // 'two' and not the 'one', so we look for an additional match at
      // one past the previous match.
      let second_digit = translate_digit(
        if let Some(following) = DIGIT_REGEX.find(&l[second.unwrap().start()+1..]) {
          following.as_str()
        } else {
          second.unwrap().as_str()
        }
      );
      first_digit * 10 + second_digit
      })
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
