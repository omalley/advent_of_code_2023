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

  fn rows_match(&self, y1: usize, y2: usize) -> usize {
    let mut smudges = 0;
    for x in 0..self.width {
      if self.locations[y1][x] != self.locations[y2][x] {
        smudges += 1;
      }
    }
    smudges
  }

  fn vertical_reflection_at(&self, at: usize) -> usize {
    let mut lower = at;
    let mut upper = at + 1;
    let mut smudges = 0;
    while upper < self.height {
      smudges += self.rows_match(lower, upper);
      if lower == 0 {
        break;
      }
      lower -= 1;
      upper += 1;
    }
    smudges
  }

  fn columns_match(&self, x1: usize, x2: usize) -> usize {
    let mut smudges = 0;
    for y in 0..self.height {
      if self.locations[y][x1] != self.locations[y][x2] {
        smudges += 1;
      }
    }
    smudges
  }

  fn horizonal_reflection_at(&self, at: usize) -> usize {
    let mut lower = at;
    let mut upper = at + 1;
    let mut smudges = 0;
    while upper < self.width {
      smudges += self.columns_match(lower, upper);
      if lower == 0 {
        break;
      }
      lower -= 1;
      upper += 1;
    }
    smudges
  }

  fn find_reflection(&self, smudges: usize) -> usize {
    for y in 0..self.height - 1 {
      if self.vertical_reflection_at(y) == smudges {
        return (y + 1) * 100
      }
    }
    for x in 0..self.width - 1 {
      if self.horizonal_reflection_at(x) == smudges{
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
  input.iter().map(|m| m.find_reflection(0)).sum()
}

pub fn part2(input: &[Map]) -> usize {
  input.iter().map(|m| m.find_reflection(1)).sum()
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
    assert_eq!(400, part2(&generator(INPUT)));
  }
}
