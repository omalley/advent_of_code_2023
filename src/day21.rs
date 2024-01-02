use std::collections::{HashMap, HashSet};
use std::ops;
use std::ops::Range;
use smallvec::SmallVec;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
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
type Time = u32;

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct Location {
  x: Coordinate,
  y: Coordinate,
}

impl ops::Add for Location {
  type Output = Location;

  fn add(self, rhs: Location) -> Self::Output {
    Location{x: self.x + rhs.x, y: self.y + rhs.y}
  }
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

  fn contains(&self, location: Location) -> bool {
    self.width.contains(&location.x) && self.height.contains(&location.y)
  }

  fn get_spot(&self, location: Location) -> Spot {
    self.spots[location.y.rem_euclid(self.height.end) as usize]
        [location.x.rem_euclid(self.width.end) as usize]
  }

  fn convert_to_grid(&self, location: Location) -> GridOffset {
    GridOffset{x: location.x.div_euclid(self.width.end), y: location.y.div_euclid(self.height.end)}
  }

  fn next<const LIMITLESS: bool>(&self, spot: Location) -> SmallVec<[Location;4]> {
    let mut result = SmallVec::new();
    for dir in [Location{x:1, y:0}, Location{x:0, y:1}, Location{x:-1, y:0}, Location{x:0, y:-1}] {
      let new = spot + dir;
      if (LIMITLESS || self.contains(new)) && self.get_spot(new) != Spot::Rock{
        result.push(new);
      }
    }
    result
  }

  fn print_map(&self, locations: &HashSet<Location>) {
    let min_x = locations.iter().map(|l| l.x).min().unwrap_or(0);
    let min_y = locations.iter().map(|l| l.y).min().unwrap_or(0);
    let max_x = locations.iter().map(|l| l.x).max().unwrap_or(0);
    let max_y = locations.iter().map(|l| l.y).max().unwrap_or(0);
    for y in min_y..=max_y {
      if y.rem_euclid(self.height.end) == 0 {
        for x in min_x..=max_x {
          if x.rem_euclid(self.width.end) == 0 {
            print!("-");
          }
          print!("-");
        }
        println!();
      }
      for x in min_x..=max_x {
        if x.rem_euclid(self.width.end) == 0 {
          print!("|");
        }
        if locations.contains(&Location{x, y}) {
          print!("0");
        } else {
          match self.get_spot(Location{x,y}) {
            Spot::Garden => print!("."),
            Spot::Rock => print!("#"),
            Spot::Start => print!("S"),
          }
        }
      }
      println!();
    }
  }

  fn moves<const LIMITLESS: bool>(&self, dist: Time) -> usize {
    let mut frontier : HashSet<Location> = HashSet::new();
    let mut done = [(); 2].map(|_| HashMap::new());
    let mut first = HashMap::new();
    frontier.insert(self.start.clone());
    first.insert(GridOffset{x:0, y:0}, GridSummary::init(0));
    for t in 0..dist {
      let mut next = HashSet::new();
      for loc in frontier.into_iter() {
        for n in self.next::<LIMITLESS>(loc) {
          done[t as usize % 2].entry(n.clone()).or_insert_with(|| {
            first.entry(GridOffset{x: n.x / self.width.end, y: n.y / self.height.end})
                .or_insert_with(|| GridSummary::init(t));
            next.insert(n);
          });
        }
      }
      frontier = next;
    }
    //self.print_map(&done[(dist + 1) % 2]);
    done[(dist as usize + 1) % 2].len()
  }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct GridOffset {
  x: Coordinate,
  y: Coordinate,
}

#[derive(Clone,Debug)]
struct GridSummary {
  time: Time,
  counts: Vec<usize>,
}

impl GridSummary {
  fn init(time: Time) -> Self {
    GridSummary{ time, counts: Vec::new() }
  }

  fn count_squares(&self, time: usize) -> usize {
    if time < self.counts.len() {
      self.counts[time]
    } else if self.counts.len() >= 2 {
      let len = self.counts.len();
      self.counts[(len - 2) + ((len % 2) + time) % 2]
    } else {
      panic!("Not enough time values!")
    }
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input)
    .unwrap() // panics on error
}

pub fn part1(input: &Map) -> usize {
  input.moves::<false>(64)
}

pub fn part2(_input: &Map) -> usize {
  //input.grid_move(26501365)
  0
}

#[cfg(test)]
mod tests {
  use crate::day21::{generator};

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
    let input = generator(INPUT);
    assert_eq!(16, input.moves::<false>(6));
  }

  #[test]
  fn test_part2() {
    let input = generator(INPUT);
    assert_eq!(668697, input.moves::<true>(160));
    //assert_eq!(50, input.moves::<true>(10));
    //assert_eq!(50, input.grid_move(10));
    //assert_eq!(668697, input.grid_move(1000));
  }
}
