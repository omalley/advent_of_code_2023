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

  fn match_rows(&self, y1: usize, y2: usize, get: impl Fn(usize, usize) -> Location,
                width: usize) -> usize {
    let mut smudges = 0;
    for x in 0..width {
      if get(x, y1) != get(x, y2) {
        smudges += 1;
      }
    }
    smudges
  }

  fn reflection_at(&self, y: usize, smudges: usize, get: impl Fn(usize, usize) -> Location,
                   height: usize, width:  usize) -> bool {
    let mut lower = y;
    let mut upper = y + 1;
    let mut smudge_count = 0;
    while upper < height {
      smudge_count += self.match_rows(lower, upper, &get, width);
      if lower == 0 || smudge_count > smudges {
        break;
      }
      lower -= 1;
      upper += 1;
    }
    smudges == smudge_count
  }

  fn locate_reflection(&self, smudges: usize, get: impl Fn(usize, usize) -> Location,
                       height: usize, width: usize) -> Option<usize> {
    for y in 0..height - 1 {
      if self.reflection_at(y, smudges, &get, height, width) {
        return Some(y + 1);
      }
    }
    None
  }

  fn find_reflection(&self, smudges: usize) -> usize {
    if let Some(ans) = self.locate_reflection(smudges,
                                              |x, y| self.locations[y][x],
                                               self.height, self.width) {
      return ans * 100
    }
    if let Some(ans) = self.locate_reflection(smudges,
                                              |x, y| self.locations[x][y],
                                              self.width, self.height) {
      return ans
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
