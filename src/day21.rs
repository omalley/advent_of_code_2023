use std::collections::HashSet;
use std::ops::Range;
use smallvec::SmallVec;

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Spot {
  Garden,
  Rock,
  Start,
}

impl Spot {
  fn from_char(ch: char) -> Result<Spot,String> {
    match ch {
      'S' => Ok(Spot::Start),
      '.' => Ok(Spot::Garden),
      '#' => Ok(Spot::Rock),
      _ => Err(format!("unknown character - {ch}")),
    }
  }
}

type Coordinate = i32;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
pub struct Location {
  x: Coordinate,
  y: Coordinate,
}

#[derive(Clone,Debug)]
pub struct Map {
  spots: Vec<Vec<Spot>>,
  start: Location,
  width: Range<Coordinate>,
  height: Range<Coordinate>,
}

impl Map {
  fn from_str(s: &str) -> Result<Self,String> {
    let mut start : Option<Location> = None;
    let spots = s.lines().enumerate()
        .map(|(y, line)| line.chars().enumerate()
            .map(|(x, ch)| {
              let s = Spot::from_char(ch);
              if let Ok(Spot::Start) = s {
                start = Some(Location{x: x as i32, y: y as i32})
              }
              s })
            .collect::<Result<Vec<Spot>,String>>())
        .collect::<Result<Vec<Vec<Spot>>,String>>()?;
    let start = start.ok_or("No start".to_string())?;
    let width = 0..(spots[0].len() as Coordinate);
    let height = 0..(spots.len() as Coordinate);
    Ok(Map{spots, start, width, height})
  }

  fn next(&self, spot: &Location) -> SmallVec<[Location;4]> {
    let mut result = SmallVec::new();
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
      let new = Location{x: spot.x + dir.0, y: spot.y + dir.1};
      if self.width.contains(&new.x) && self.height.contains(&new.y) &&
          self.spots[new.y as usize][new.x as usize] != Spot::Rock {
        result.push(new);
      }
    }
    result
  }

  fn moves(&self, dist: usize) -> usize {
    let mut current : HashSet<Location> = HashSet::new();
    current.insert(self.start.clone());
    for _ in 0..dist {
      let mut next = HashSet::new();
      for loc in &current {
        for n in self.next(loc) {
          next.insert(n);
        }
      }
      current = next;
    }
    current.len()
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input)
    .unwrap() // panics on error
}

pub fn part1(input: &Map) -> usize {
  input.moves(64)
}

pub fn part2(_input: &Map) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use crate::day21::{generator, part2};

  const INPUT: &str =
"...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

  #[test]
  fn test_part1() {
    assert_eq!(16, generator(INPUT).moves(6));
  }

  #[test]
  fn test_part2() {
    assert_eq!(0, part2(&generator(INPUT)));
  }
}
