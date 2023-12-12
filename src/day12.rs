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
  runs_at: SmallVec<[usize; 4]>,
  length: usize,
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

  fn count_matches(&self) -> usize {
    let total_length = self.springs.len();
    //println!("input = {:?}, length = {total_length}", self);
    let mut pending: Vec<PartialSolution> = Vec::new();
    pending.push(PartialSolution::default());
    let mut solution_count = 0;
    while let Some(next) = pending.pop() {
      //println!("popped: {:?}", next);
      // Have we placed all of the broken runs?
      if next.runs_at.len() == self.broken_counts.len() {
        // We are successful if there aren't any remaining broken springs.
        if Self::is_not_broken(&self.springs[next.length..]) {
          solution_count += 1;
          //println!("good solutions now {solution_count}");
        }
      } else {
        let current = next.runs_at.len();
        let max_position = total_length
            - self.broken_counts[current..].iter().sum::<usize>()
            - (self.broken_counts.len() - current - 1);
        //println!("max posn = {max_position}");
        for posn in next.length..=max_position {
          let end_posn = posn + self.broken_counts[current];
          if Self::is_not_broken(&self.springs[next.length..posn]) &&
              Self::is_broken(&self.springs[posn..end_posn]) &&
              Self::ends_run(&self.springs, end_posn) {
            let mut next_state = next.clone();
            next_state.runs_at.push(posn);
            next_state.length = (end_posn + 1).min(total_length);
            pending.push(next_state);
          }
        }
      }
    }
    //println!("answer = {solution_count}");
    solution_count
  }
}

pub fn generator(input: &str) -> Vec<Record> {
  input.lines().map(Record::from_str).collect::<Result<Vec<Record>,String>>()
      .unwrap() // panic on error
}

pub fn part1(input: &Vec<Record>) -> usize {
  input.iter().map(|r| r.count_matches()).sum()
}

pub fn part2(_input: &Vec<Record>) -> usize {
  0
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
  }

  #[test]
  fn test_part2() {
    assert_eq!(0, part2(&generator(INPUT)));
  }
}
