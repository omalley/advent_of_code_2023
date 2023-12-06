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
    let swing = f64::sqrt(time * time - 4.0 * self.record as f64) / 2.0;
    if self.time % 2 == 0 {
      swing.ceil() as u64 * 2 - 1
    } else {
      (swing - 0.5).ceil() as u64 * 2
    }
  }
}

pub fn generator(input: &str) -> Vec<Race> {
  Race::from_str(input)
      .unwrap() // panics on error
}

pub fn part1(races: &[Race]) -> u64 {
  races.iter().map(|r| r.find_wins()).product()
}

fn append_number(base: u64, right: u64) -> u64 {
  base * 10_u64.pow(right.checked_ilog10().unwrap_or(0)+1) + right
}

pub fn part2(races: &[Race]) -> u64 {
  let mut time: u64 = 0;
  let mut record: u64 = 0;
  for r in races {
    time = append_number(time, r.time);
    record = append_number(record, r.record);
  }
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
