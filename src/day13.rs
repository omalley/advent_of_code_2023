#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Location {
  Ash,
  Rock,
}

impl Location {
  fn from_char(ch: char) -> Result<Self, String> {
    match ch {
      '.' => Ok(Location::Ash),
      '#' => Ok(Location::Rock),
      _ => Err(format!("Unknown character: {ch}")),
    }
  }
}

#[derive(Clone,Debug)]
pub struct Map {
  locations: Vec<Vec<Location>>,
  width: usize,
  height: usize,
}

impl Map {
  fn from_str(input: &str) -> Result<Self, String> {
    let locations = input.lines()
        .map(|l| l.chars().map(Location::from_char)
            .collect::<Result<Vec<Location>,String>>())
        .collect::<Result<Vec<Vec<Location>>,String>>()?;
    let width = locations[0].len();
    let height = locations.len();
    Ok(Map{locations, width, height})
  }

  fn vertical_reflection_at(&self, at: usize) -> bool {
    let mut lower = at;
    let mut upper = at + 1;
    while upper < self.height {
      if self.locations[lower] != self.locations[upper] {
        return false;
      }
      if lower == 0 {
        break;
      }
      lower -= 1;
      upper += 1;
    }
    true
  }

  fn columns_match(&self, x1: usize, x2: usize) -> bool {
    for y in 0..self.height {
      if self.locations[y][x1] != self.locations[y][x2] {
        return false;
      }
    }
    true
  }

  fn horizonal_reflection_at(&self, at: usize) -> bool {
    let mut lower = at;
    let mut upper = at + 1;
    while upper < self.width {
      if !self.columns_match(lower, upper) {
        return false;
      }
      if lower == 0 {
        break;
      }
      lower -= 1;
      upper += 1;
    }
    true
  }

  fn find_reflection(&self) -> usize {
    for y in 0..self.height - 1 {
      if self.vertical_reflection_at(y) {
        return (y + 1) * 100
      }
    }
    for x in 0..self.width - 1 {
      if self.horizonal_reflection_at(x) {
        return x + 1
      }
    }
    0
  }
}

pub fn generator(input: &str) -> Vec<Map> {
  input.split("\n\n").map(Map::from_str).collect::<Result<Vec<Map>,String>>()
      .unwrap() // panic on error
}

pub fn part1(input: &[Map]) -> usize {
  input.iter().map(|m| m.find_reflection()).sum()
}

pub fn part2(input: &[Map]) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use crate::day13::{generator, part1, part2};

  const INPUT: &str =
"#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

  #[test]
  fn test_part1() {
    assert_eq!(405, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(525152, part2(&generator(INPUT)));
  }
}
