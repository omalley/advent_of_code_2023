#[derive(Clone,Copy,Debug,PartialEq)]
pub enum RockKind {
  RoundRock,
  CubeRock,
}

#[derive(Clone,Debug,PartialEq)]
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

#[derive(Clone,Debug)]
pub struct Map {
  columns: Vec<Vec<Rock>>,
  width: usize,
  height: usize,
}

impl Map {
  fn from_str(input: &str) -> Result<Self, String> {
    let mut width = 0;
    let mut height = 0;
    let mut columns = Vec::new();
    for (y, row) in input.lines().enumerate() {
      height = y + 1;
      if y == 0 {
        width = row.chars().count();
        columns.append(&mut vec![Vec::new(); width]);
      }
      for (x, ch) in row.chars().enumerate() {
        if let Some(r) = Rock::from_char(ch, x, y)? {
          columns[x].push(r);
        }
      }
    }
    Ok(Map{columns, width, height})
  }

  fn fall_north(&mut self) {
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

  fn get_weight(&self) -> usize {
    let mut result = 0;
    for col in &self.columns {
      for rock in col {
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

pub fn part2(_input: &Map) -> usize {
  0
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

  #[test]
  fn test_part2() {
    assert_eq!(0, part2(&generator(INPUT)));
  }
}
