use std::collections::HashMap;

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
pub enum RockKind {
  RoundRock,
  CubeRock,
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Rock {
  kind: RockKind,
  x: usize,
  y: usize,
}

impl Rock {
  fn from_char(ch: char, x: usize, y: usize) -> Result<Option<Self>, String> {
    match ch {
      'O' => Ok(Some(Rock{kind: RockKind::RoundRock, x, y})),
      '#' => Ok(Some(Rock{kind: RockKind::CubeRock, x, y})),
      '.' => Ok(None),
      _ => Err(format!("Unknown character: {ch} at {x}, {y}")),
    }
  }
}

#[derive(Clone,Debug,Hash,Eq,PartialEq)]
pub struct Map {
  columns: Vec<Vec<Rock>>,
  rows: Vec<Vec<Rock>>,
  width: usize,
  height: usize,
}

impl Map {
  fn from_str(input: &str) -> Result<Self, String> {
    let mut rows = Vec::new();
    let mut width = 0;
    for (y, row) in input.lines().enumerate() {
      let mut next_row = Vec::new();
      for (x, ch) in row.chars().enumerate() {
        width = width.max(x + 1);
        if let Some(rock) = Rock::from_char(ch, x, y)? {
          next_row.push(rock);
        }
      }
      rows.push(next_row);
    }
    let height = rows.len();
    let columns = vec![Vec::new(); width];
    Ok(Map{columns, width, height, rows})
  }

  /// rotate the rocks into the columns
  fn rotate_to_columns(&mut self) {
    for row in self.rows.iter_mut() {
      while let Some(rock) = row.pop() {
        self.columns[rock.x].push(rock);
      }
    }
  }

  fn fall_north(&mut self) {
    self.rotate_to_columns();
    for col in self.columns.iter_mut() {
      let mut next = 0;
      for rock in col.iter_mut() {
        match rock.kind {
          RockKind::RoundRock => rock.y = next,
          RockKind::CubeRock => next = rock.y,
        }
        next += 1;
      }
    }
  }

  /// rotate the rocks into the rows
  fn rotate_to_rows(&mut self) {
   for col in self.columns.iter_mut() {
      while let Some(rock) = col.pop() {
        self.rows[rock.y].push(rock);
      }
    }
  }

  fn fall_west(&mut self) {
    self.rotate_to_rows();
    for row in self.rows.iter_mut() {
      let mut next = 0;
      for rock in row.iter_mut() {
        match rock.kind {
          RockKind::RoundRock => rock.x = next,
          RockKind::CubeRock => next = rock.x,
        }
        next += 1;
      }
    }
  }

  fn fall_south(&mut self) {
    self.rotate_to_columns();
    for col in self.columns.iter_mut() {
      let mut prev = self.height;
      for rock in col.iter_mut().rev() {
        prev -= 1;
        match rock.kind {
          RockKind::RoundRock => rock.y = prev,
          RockKind::CubeRock => prev = rock.y,
        }
      }
    }
  }

  fn fall_east(&mut self) {
    self.rotate_to_rows();
    for row in self.rows.iter_mut() {
      let mut prev = self.width;
      for rock in row.iter_mut().rev() {
        prev -= 1;
        match rock.kind {
          RockKind::RoundRock => rock.x = prev,
          RockKind::CubeRock => prev = rock.x,
        }
      }
    }
  }

  fn cycle(&mut self) {
    self.fall_north();
    self.fall_west();
    self.fall_south();
    self.fall_east();
  }

  fn get_weight(&self) -> usize {
    let mut result = 0;
    for col in &self.columns {
      for rock in col {
        if rock.kind == RockKind::RoundRock {
          result += self.height - rock.y;
        }
      }
    }
    for row in &self.rows {
      for rock in row {
        if rock.kind == RockKind::RoundRock {
          result += self.height - rock.y;
        }
      }
    }
    result
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input)
      .unwrap() // panic on error
}

pub fn part1(input: &Map) -> usize {
  let mut work = input.clone();
  work.fall_north();
  work.get_weight()
}

const PART2_REPETITIONS: usize = 1_000_000_000;

pub fn part2(input: &Map) -> usize {
  let mut work = input.clone();
  let mut cache: HashMap<Map, usize> = HashMap::new();
  let mut found_cycle: Option<(usize, usize)> = None;
  for cycle in 0..PART2_REPETITIONS {
    work.cycle();
    if let Some(prev) = cache.insert(work.clone(), cycle + 1) {
      found_cycle = Some((prev, cycle + 1 - prev));
      break;
    }
  }
  cache.clear();
  // If we found a loop, simulate the remaining cycles.
  if let Some((base, length)) = found_cycle {
    let remaining = (PART2_REPETITIONS - base) % length;
    for _ in 0..remaining {
      work.cycle();
    }
  }
  work.get_weight()
}

#[cfg(test)]
mod tests {
  use crate::day14::{generator, part1, part2};

  const INPUT: &str =
"O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";

  #[test]
  fn test_part1() {
    assert_eq!(136, part1(&generator(INPUT)));
  }

  const THREE_CYCLE_OUTPUT: &str =
".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O";

  #[test]
  fn test_3_cycles() {
    let expected = generator(&THREE_CYCLE_OUTPUT);
    let mut input = generator(&INPUT);
    for _ in 0..3 {
      input.cycle();
    }
    assert_eq!(expected, input);
  }

  #[test]
  fn test_part2() {
    assert_eq!(64, part2(&generator(INPUT)));
  }
}
