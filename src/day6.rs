#[derive(Debug)]
pub struct Race {
  time: u64,
  record: u64,
}

fn parse_ints(s: Option<&str>, field_name: &str) -> Result<Vec<u64>, String> {
  s.ok_or(format!("missing field {field_name}"))?
      .split_whitespace()
      .skip(1)
      .map(|w| w.parse().map_err(|_| format!("Can't parse integer - {}", s.unwrap())))
      .collect()
}

impl Race {
  fn from_str(s: &str) -> Result<Vec<Self>,String> {
    let mut lines = s.lines();
    let times = parse_ints(lines.next(), "time")?;
    let records = parse_ints(lines.next(), "records")?;
    Ok(times.iter().zip(records.iter())
        .map(|(t, r)| Race{time: *t, record: *r}).collect())
  }

  fn find_wins(&self) -> u64 {
    let time = self.time as f64;
    let mid = time / 2.0;
    let swing = f64::sqrt(time * time - 4.0 * self.record as f64) / 2.0;
    (mid + swing).ceil() as u64 - (mid - swing + 1.0).floor() as u64
  }
}

pub fn generator(input: &str) -> Vec<Race> {
  Race::from_str(input)
      .unwrap() // panics on error
}

pub fn part1(races: &[Race]) -> u64 {
  races.iter().map(|r| r.find_wins()).product()
}

fn munge_numbers(seq: &[u64]) -> u64 {
  let mut result: u64 = 0;
  for n in seq {
    let power = if *n == 0 {
      1
    } else {
      n.ilog10() + 1
    };
    result = result * 10u64.pow(power) + n;
  }
  result
}

pub fn part2(races: &[Race]) -> u64 {
  let time = munge_numbers(&races.iter().map(|r| r.time).collect::<Vec<u64>>());
  let record = munge_numbers(&races.iter().map(|r| r.record).collect::<Vec<u64>>());
  Race{time, record}.find_wins()
}

#[cfg(test)]
mod tests {
  use crate::day6::{generator, part1, part2};

  const INPUT: &str =
"Time:      7  15   30
Distance:  9  40  200";

  #[test]
  fn test_part1() {
    assert_eq!(288, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(71503, part2(&generator(INPUT)));
  }
}
