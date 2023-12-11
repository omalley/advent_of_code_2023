#[derive(Clone,Copy,Debug,Default)]
pub struct StretchyCoordinates {
  real: usize,
  stretch: usize,
}

impl StretchyCoordinates {
  fn abs_diff(&self, other: StretchyCoordinates) -> StretchyCoordinates {
    StretchyCoordinates{real: self.real.abs_diff(other.real),
      stretch: self.stretch.abs_diff(other.stretch)}
  }

  fn plus(&self, other: StretchyCoordinates) -> StretchyCoordinates {
    StretchyCoordinates{real: self.real + other.real,
      stretch: self.stretch + other.stretch}
  }
}

#[derive(Clone,Debug)]
pub struct Galaxy {
  x: StretchyCoordinates,
  y: StretchyCoordinates,
}

impl Galaxy {
  fn distance(&self, other: &Galaxy) -> StretchyCoordinates {
    self.x.abs_diff(other.x).plus(
      self.y.abs_diff(other.y))
  }
}

#[derive(Clone,Debug)]
pub struct Map {
  galaxies: Vec<Galaxy>,
}

impl Map {
  fn from_str(input: &str) -> Map {
    let mut galaxies = Vec::new();
    let mut expanding_rows = Vec::new();
    let mut empty_columns = Vec::new();
    for (y, row) in input.lines().enumerate() {
      // Set the empty columns up to be the right length
      if y == 0 {
        for _ in row.chars() {
          empty_columns.push(true);
        }
      }
      let mut is_empty = true;
      for (x, ch) in row.chars().enumerate() {
        if ch == '#' {
          galaxies.push(Galaxy{x: StretchyCoordinates{real:x, stretch: 0},
            y: StretchyCoordinates{real:y, stretch: 0}});
          is_empty = false;
          empty_columns[x] = false;
        }
      }
      if is_empty {
        expanding_rows.push(y);
      }
    }
    let expanding_columns: Vec<usize> = empty_columns.iter().enumerate()
        .filter_map(|(i, &empty)| if empty { Some(i) } else { None })
        .collect();
    for g in galaxies.iter_mut() {
      g.x.stretch = expanding_columns.partition_point(|col| *col < g.x.real);
      g.y.stretch = expanding_rows.partition_point(|row| *row < g.y.real);
    }
    Map{galaxies}
  }

  fn sum_distances(&self) -> StretchyCoordinates {
    let mut sum = StretchyCoordinates::default();
    for (i, g1) in self.galaxies.iter().enumerate() {
      for (j, g2) in self.galaxies.iter().enumerate() {
        if i < j {
          sum = sum.plus(g1.distance(g2));
        }
      }
    }
    sum
  }
}

pub fn generator(input: &str) -> StretchyCoordinates {
  Map::from_str(input).sum_distances()
}

pub fn part1(input: &StretchyCoordinates) -> usize {
  input.real + input.stretch
}

pub fn part2(input: &StretchyCoordinates) -> usize {
  input.real + input.stretch * 999_999
}

#[cfg(test)]
mod tests {
  use crate::day11::{generator, part1};

  const INPUT: &str =
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

  #[test]
  fn test_part1() {
    assert_eq!(374, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    let input = generator(INPUT);
    assert_eq!(8410, input.real + input.stretch * 99);
  }
}
