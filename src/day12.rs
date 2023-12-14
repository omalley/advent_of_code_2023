#[derive(Clone,Debug,PartialEq)]
pub enum SpringState {
  Good,
  Broken,
  Unknown,
}

impl SpringState {
  fn from_char(ch: char) -> Result<Self, String> {
    match ch {
      '.' => Ok(SpringState::Good),
      '#' => Ok(SpringState::Broken),
      '?' => Ok(SpringState::Unknown),
      _ => Err(format!("Unknown character: {ch}")),
    }
  }
}

#[derive(Clone,Debug)]
struct PartialSolution {
  finished: u32,
  length: u32,
  multiplier: usize,
}

impl PartialSolution {
  fn default() -> Self {
    PartialSolution{finished:0, length:0, multiplier: 1}
  }

  /// Advance the solution over any known good springs.
  fn skip_over_good(&mut self, springs: &[SpringState]) -> &mut Self {
    while (self.length as usize) < springs.len() &&
        springs[self.length as usize] == SpringState::Good {
      self.length += 1;
    }
    self
  }

  /// Find the length of the next sequence of broken & unknown
  /// springs.
  /// Returns the length of the range and whether it is the end.
  fn find_next_range(&self, springs: &[SpringState]) -> Option<(usize, bool)> {
    for (i, spring) in springs[self.length as usize..]
        .iter().enumerate() {
      if *spring == SpringState::Good {
        return if i == 0 { None } else { Some((i, false)) };
      }
    }
    if springs.len() == self.length as usize { None } else {
      Some (( springs.len() - self.length as usize, true))}
  }

  fn extend(&mut self, springs: usize, runs: usize) -> &mut Self {
    self.finished += runs as u32;
    self.length += springs as u32;
    self
  }

  fn multiply(&mut self, multiplier: usize) -> &mut Self {
    self.multiplier *= multiplier;
    self
  }
}

#[derive(Clone,Debug)]
pub struct Record {
  springs: Vec<SpringState>,
  broken_counts: Vec<usize>,
}

impl Record {
  fn from_str(input: &str) -> Result<Self, String> {
    let (spring_str, count_str) =
        input.split_once(' ').ok_or("Can't find separator")?;
    let springs = spring_str.chars().map(SpringState::from_char)
        .collect::<Result<Vec<SpringState>,String>>()?;
    let broken_counts = count_str.split(',')
        .map(|x| x.parse::<usize>().map_err(|_| format!("Can't parse integer: {x}")))
        .collect::<Result<Vec<usize>,String>>()?;
    Ok(Record{springs, broken_counts})
  }

  fn is_not_broken(springs: &[SpringState]) -> bool {
    springs.iter().all(|s| *s != SpringState::Broken)
  }

  fn is_all_unknown(springs: &[SpringState]) -> bool {
    springs.iter().all(|s| *s == SpringState::Unknown)
  }

  /// Count the number of combinations given a series of unknown locations.
  fn count_unknown_range(spring_len: usize, broken_runs: &[usize]) -> usize {
    let broken_len = broken_runs.iter().sum::<usize>() + broken_runs.len() - 1;
    if broken_len > spring_len {
      0
    } else {
      match broken_runs.len() {
        0 => 0,
        1 => spring_len - broken_runs[0] + 1,
        2 => {
          let n = spring_len - broken_len + 1;
          n * (n + 1) / 2
        },
        _ => {
          let tail_length = broken_runs[1..].iter().sum::<usize>() + broken_runs.len() - 2;
          let head_length = broken_runs[0] + 1;
          let mut result = 0;
          for offset in 0..=spring_len-tail_length-head_length {
            result += Self::count_unknown_range(spring_len - offset - head_length,
                                                &broken_runs[1..]);
          }
          result
        }
      }
    }
  }

  /// Count the combinations given a series of springs (only broken and unknown) and
  /// a set of runs to match. All of the springs and broken_counts must be accounted
  /// for.
  fn count_mixed_range(springs: &[SpringState], broken_counts: &[usize]) -> usize {
    if springs.is_empty() || broken_counts.is_empty() {
      0
    } else if Self::is_all_unknown(springs) {
      Self::count_unknown_range(springs.len(), broken_counts)
    } else {
      let mut result = 0;
      let min_length = broken_counts.iter().sum::<usize>() + (broken_counts.len() - 1);
      if min_length <= springs.len() {
        let length = broken_counts[0];
        for posn in 0..=springs.len() - min_length {
          if broken_counts.len() == 1 {
            if Self::is_all_unknown(&springs[posn + length..]) {
              result += 1;
            }
          } else if springs[posn + length] == SpringState::Unknown {
            result += Self::count_mixed_range(&springs[posn + length + 1..],
                                              &broken_counts[1..]);
          }
          // If the current spring is broken, this is the last position we can start at.
          if springs[posn] == SpringState::Broken {
            break
          }
        }
      }
      result
    }
  }

  fn count_matches(&self) -> usize {
    let mut pending: Vec<PartialSolution> = Vec::new();
    pending.push(PartialSolution::default().skip_over_good(&self.springs).clone());
    let mut solution_count = 0;
    while let Some(next) = pending.pop() {
      // Have we placed all of the broken runs?
      if next.finished as usize == self.broken_counts.len() {
        // We are successful if there aren't any remaining broken springs.
        if Self::is_not_broken(&self.springs[next.length as usize ..]) {
          solution_count += next.multiplier;
        }
      } else if let Some((range, is_end)) = next.find_next_range(&self.springs) {
        let current = next.finished as usize;
        if is_end {
          // If this is the final range, it has to match the rest of the runs.
          solution_count += next.multiplier * Self::count_mixed_range(
            &self.springs[next.length as usize..], &self.broken_counts[current..]);
        } else {
          // try out not putting any runs here
          if Self::is_all_unknown(&self.springs[next.length as usize..next.length as usize+range]) {
            pending.push(next.clone().extend(range, 0).skip_over_good(&self.springs).clone());
          }
          let mut required_space = 0;
          for included_run in current..self.broken_counts.len() {
            required_space += self.broken_counts[included_run];
            // include the extra good space
            if included_run > current {
              required_space += 1;
            }
            // If we don't fit in the range, we can stop trying to add more runs.
            if required_space > range {
              break;
            }
            let combinations = Self::count_mixed_range(
              &self.springs[next.length as usize..next.length as usize + range],
              &self.broken_counts[current..=included_run]);
            if combinations != 0 {
              pending.push(next.clone().extend(range, included_run - current + 1)
                  .multiply(combinations).skip_over_good(&self.springs).clone());
            }
          }
        }
      }
    }
    solution_count
  }

  fn extend(&self, factor: usize) -> Self {
    let mut springs = self.springs.clone();
    let mut broken_counts = self.broken_counts.clone();
    for _ in 1..factor {
      springs.push(SpringState::Unknown);
      springs.append(&mut self.springs.clone());
      broken_counts.append(&mut self.broken_counts.clone());
    }
    Record{springs, broken_counts}
  }
}

pub fn generator(input: &str) -> Vec<Record> {
  input.lines().map(Record::from_str).collect::<Result<Vec<Record>,String>>()
      .unwrap() // panic on error
}

pub fn part1(input: &[Record]) -> usize {
  input.iter().map(|r| r.count_matches()).sum()
}

pub fn part2(input: &[Record]) -> usize {
  input.iter()
      .map(|r | r.extend(5).count_matches())
      .sum()
}

#[cfg(test)]
mod tests {
  use crate::day12::{generator, part1, part2};

  const INPUT: &str =
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

  #[test]
  fn test_part1() {
    assert_eq!(21, part1(&generator(INPUT)));
    assert_eq!(15, part1(&generator("?#?????.??????. 4,1")));
  }

  #[test]
  fn test_part2() {
    assert_eq!(525152, part2(&generator(INPUT)));
  }

  #[test]
  fn extra_test() {
    assert_eq!(2, part1(&generator("????? 4")));
    assert_eq!(36, part1(&generator("??????????????? 3,4")));
    assert_eq!(84, part1(&generator("???????????????????? 3,4,5")));
    assert_eq!(35, part1(&generator("???????????????????? 3,4,5,2")));
  }
}
