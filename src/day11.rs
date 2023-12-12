#[derive(Clone,Debug)]
pub struct Galaxy {
  x: usize,
  y: usize,
}

impl Galaxy {
  fn distance(&self, other: &Galaxy) -> usize {
    self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
  }
}

#[derive(Clone,Debug)]
pub struct Map {
  galaxies: Vec<Galaxy>,
  expanding_rows: Vec<usize>,
  expanding_columns: Vec<usize>,
}

impl Map {
  fn from_str(input: &str) -> Map {
    let mut galaxies = Vec::new();
    let mut expanding_rows = Vec::new();
    let mut empty_columns = Vec::new();
    for (y, row) in input.lines().enumerate() {
      // Set the empty columns up to be the right length
      if y == 0 {
        empty_columns = vec![true; row.len()];
      }
      let mut is_empty = true;
      for (x, ch) in row.chars().enumerate() {
        if ch == '#' {
          galaxies.push(Galaxy{x,y});
          is_empty = false;
          empty_columns[x] = false;
        }
      }
      if is_empty {
        expanding_rows.push(y);
      }
    }
    let expanding_columns = empty_columns.iter().enumerate()
        .filter_map(|(i, &empty)| if empty { Some(i) } else { None })
        .collect();
    Map{galaxies, expanding_rows, expanding_columns}
  }

  fn expand(&mut self, factor: usize) {
    for g in self.galaxies.iter_mut() {
      g.x += (factor - 1) * self.expanding_columns.partition_point(|col| *col < g.x);
      g.y += (factor - 1) * self.expanding_rows.partition_point(|row| *row < g.y);
    }
  }

  fn sum_distances(&self) -> usize {
    let mut sum = 0;
    for (i, g1) in self.galaxies.iter().enumerate() {
      for (j, g2) in self.galaxies.iter().enumerate() {
        if i < j {
          sum += g1.distance(g2);
        }
      }
    }
    sum
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input)
}

pub fn part1(input: &Map) -> usize {
  let mut expanding_galaxy = input.clone();
  expanding_galaxy.expand(2);
  expanding_galaxy.sum_distances()
}

pub fn part2(input: &Map) -> usize {
  let mut expanding_galaxy = input.clone();
  expanding_galaxy.expand(1_000_000);
  expanding_galaxy.sum_distances()
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
    let mut input = generator(INPUT);
    input.expand(100);
    assert_eq!(8410, input.sum_distances());
  }
}
