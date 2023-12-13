use chrono::Local;
use smallvec::SmallVec;

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

#[derive(Clone,Debug,Default)]
struct PartialSolution {
  runs_at: SmallVec<[u16; 30]>,
  length: u16,
  multiplier: usize,
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

  fn is_broken(springs: &[SpringState]) -> bool {
    springs.iter().all(|s| *s != SpringState::Good)
  }

  /// Checks to make sure the position can end a broken run.
  /// It must either be non-broken or at the end of the Vec.
  fn ends_run(springs: &[SpringState], position: usize) -> bool {
    springs.get(position).map(|p| *p != SpringState::Broken)
        .unwrap_or(true)
  }

  /// Determine if there is a run of unknowns that is terminated with a good or
  /// the end of the sequence. We are looking for unknown substrings that will
  /// contain disjoint runs of broken springs.
  fn unknown_run(springs: &[SpringState]) -> Option<usize> {
    for (i, s) in springs.iter().enumerate() {
      match s {
        SpringState::Good => return if i == 0 { None } else { Some(i) },
        SpringState::Broken => return None,
        _ => {},
      }
    }
    if springs.is_empty() {
      None
    } else {
      Some(springs.len())
    }
  }

  /// Count the number of combinations given a series of unknown locations.
  fn count_unknown_run(spring_len: usize, broken_runs: &[usize]) -> usize {
    match broken_runs.len() {
      0 => 0,
      1 => spring_len - broken_runs[0] + 1,
      2 => {
        let n = spring_len - broken_runs.iter().sum::<usize>();
        n * (n + 1) / 2
      },
      _ => {
        let tail_length = broken_runs[1..].iter().sum::<usize>() + broken_runs.len() - 2;
        let head_length = broken_runs[0] + 1;
        let mut result = 0;
        for offset in 0..=spring_len-tail_length-head_length {
          result += Self::count_unknown_run(spring_len - offset - head_length,
                                            &broken_runs[1..]);
        }
        result
      }
    }
  }

  fn handle_unknown_run(&self, next: &PartialSolution, unknown_run: usize) -> Vec<PartialSolution> {
    let mut pending = Vec::new();
    let mut new_partial = next.clone();
    new_partial.length = (new_partial.length as usize  + unknown_run + 1)
        .min(self.springs.len()) as u16;
    pending.push(new_partial.clone());
    let start = next.runs_at.len();
    let mut current = start;
    let mut min_size = self.broken_counts[start];
    while min_size <= unknown_run {
      new_partial.multiplier = next.multiplier *
          Self::count_unknown_run(unknown_run, &self.broken_counts[start..=current]);
      new_partial.runs_at = next.runs_at.clone();
      new_partial.runs_at.extend_from_slice(&vec![next.length; current - start + 1]);
      pending.push(new_partial.clone());
      current += 1;
      if current == self.broken_counts.len() {
        break;
      }
      min_size += 1 + self.broken_counts[current];
    }
    pending
  }

  fn count_matches(&self) -> usize {
    let total_length = self.springs.len();
    let mut pending: Vec<PartialSolution> = Vec::new();
    pending.push(PartialSolution{multiplier: 1,.. PartialSolution::default()});
    let mut solution_count = 0;
    while let Some(next) = pending.pop() {
      // Have we placed all of the broken runs?
      if next.runs_at.len() == self.broken_counts.len() {
        // We are successful if there aren't any remaining broken springs.
        if Self::is_not_broken(&self.springs[next.length as usize ..]) {
          solution_count += next.multiplier;
        }
      // handle the special case of a terminated series of unknowns
      } else if let Some(unknown_run) =
          Self::unknown_run(&self.springs[next.length as usize..]) {
        pending.append(&mut self.handle_unknown_run(&next, unknown_run));
      } else {
        let current = next.runs_at.len();
        let max_position = total_length
            - self.broken_counts[current..].iter().sum::<usize>()
            - (self.broken_counts.len() - current - 1);
        for posn in next.length as usize..=max_position {
          let end_posn = posn + self.broken_counts[current];
          if Self::is_not_broken(&self.springs[next.length as usize..posn]) &&
              Self::is_broken(&self.springs[posn..end_posn]) &&
              Self::ends_run(&self.springs, end_posn) {
            let mut next_state = next.clone();
            next_state.runs_at.push(posn as u16);
            next_state.length = (end_posn + 1).min(total_length) as u16;
            pending.push(next_state);
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
  input.iter().enumerate().map(|(i, r) | {
    let time = Local::now();
    println!("run {i} - {}", time.format("%d/%m/%Y %H:%M"));
    r.extend(5).count_matches()
  }).sum()
}

#[cfg(test)]
mod tests {
  use crate::day12::{generator, part1, part2, Record};

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

  #[test]
  fn test_count_unknown() {
    assert_eq!(2, Record::count_unknown_run(5, &vec![4]));
    assert_eq!(36, Record::count_unknown_run(15, &vec![3, 4]));
    assert_eq!(84, Record::count_unknown_run(20, &vec![3, 4, 5]));
    assert_eq!(35, Record::count_unknown_run(20, &vec![3, 4, 5, 2]));
  }
}
