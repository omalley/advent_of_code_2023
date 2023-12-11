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
        for _ in row.chars() {
          empty_columns.push(true);
        }
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

  fn expand(&mut self) {
    for g in self.galaxies.iter_mut() {
      g.x += self.expanding_columns.partition_point(|col| *col < g.x);
      g.y += self.expanding_rows.partition_point(|row| *row < g.y);
    }
  }
}

pub fn generator(input: &str) -> Map {
  let mut result = Map::from_str(input);
  result.expand();
  result
}

pub fn part1(input: &Map) -> usize {
  let mut sum = 0;
  for (i, g1) in input.galaxies.iter().enumerate() {
    for (j, g2) in input.galaxies.iter().enumerate() {
      if i < j {
        sum += g1.distance(g2);
      }
    }
  }
  sum
}

pub fn part2(input: &Map) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use crate::day11::{generator, part1, part2};

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
    assert_eq!(0, part2(&generator(INPUT)));
  }
}
