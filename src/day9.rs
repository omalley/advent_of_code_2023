pub fn generator(input: &str) -> Vec<Vec<i32>> {
  input.lines()
    .map(|line| line.split_whitespace().map(|n| n.parse().unwrap()).collect())
    .collect()
}

#[derive(Default)]
pub struct HistoryLine {
  values: Vec<i32>,
}

impl HistoryLine {
  fn _from(line: &str, reverse: bool) -> HistoryLine {
    let nums: Vec<i32> = line.split_whitespace()
      .map(|n| n.parse().unwrap()).collect();
    if reverse {
      HistoryLine::from_nums(&mut nums.iter().rev())
    } else {
      HistoryLine::from_nums(&mut nums.iter())
    }
  }

  fn from_nums(nums: &mut dyn Iterator<Item=&i32>) -> HistoryLine {
    let mut hl = HistoryLine::default();
    hl.add(nums);
    hl
  }

  fn add(&mut self, itr: &mut dyn Iterator<Item=&i32>) {
    for n in itr {
      let mut next = *n;
      for v in self.values.iter_mut() {
        let park = *v;
        *v = next;
        next -= park;
      }
      self.values.push(next);
    }
  }

  fn next_value(&self) -> i32 {
    self.values.iter().sum()
  }
}

fn compute(history_lines: &[Vec<i32>], reverse: bool) -> i64 {
  history_lines.iter().map(|nums| {
    let hl = if reverse {
      HistoryLine::from_nums(&mut nums.iter().rev())
    } else {
      HistoryLine::from_nums(&mut nums.iter())
    };
    hl.next_value() as i64
  })
    .sum()
}

pub fn part1(history_lines: &[Vec<i32>]) -> i64 {
  compute(history_lines, false)
}

pub fn part2(history_lines: &[Vec<i32>]) -> i64 {
  compute(history_lines, true)
}

#[cfg(test)]
mod tests {
  use crate::day9::{generator, HistoryLine, part1, part2};

  fn input() -> String {
    "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".to_string()
  }

  #[test]
  fn test_generator() {
    let lines = generator(&input());
    assert_eq!(3, lines.len());
  }

  #[test]
  fn test_predict() {
    let mut l = HistoryLine::_from("0 3", false);
    assert_eq!(l.values, vec![3, 3]);
    l.add(&6);
    assert_eq!(l.values, vec![6, 3, 0]);
    l.add(&9);
    assert_eq!(l.values, vec![9, 3, 0, 0]);
    assert_eq!(12, l.next_value());

    let mut l = HistoryLine::_from("10 13", false);
    assert_eq!(l.values, vec![13, 3]);
    l.add(&16);
    assert_eq!(l.values, vec![16, 3, 0]);
    l.add(&21);
    assert_eq!(l.values, vec![21, 5, 2, 2]);
    l.add(&30);
    assert_eq!(l.values, vec![30, 9, 4, 2, 0]);
    l.add(&45);
    assert_eq!(l.values, vec![45, 15, 6, 2, 0, 0]);
    assert_eq!(l.next_value(), 68);

    let l = HistoryLine::_from("10 13 16 21 30 45", false);
    assert_eq!(l.next_value(), 68);
  }

  #[test]
  fn test_part1() {
    let lines = generator(&input());
    assert_eq!(114, part1(&lines));
  }

  #[test]
  fn test_backwards() {
    let l = HistoryLine::_from("10 13 16 21 30 45", true);
    assert_eq!(l.next_value(), 5);
  }

  #[test]
  fn test_part2() {
    let lines = generator(&input());
    assert_eq!(2, part2(&lines));
  }
}