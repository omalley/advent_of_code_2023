use std::cmp::Ordering;
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

type Position = i32;
type Time = u32;

#[derive(Clone,Copy,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
pub struct Coordinate {
  x: Position,
  y: Position,
}

impl ops::Add for Coordinate {
  type Output = Coordinate;

  fn add(self, rhs: Coordinate) -> Self::Output {
    Coordinate {x: self.x + rhs.x, y: self.y + rhs.y}
  }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
struct GridCoordinate {
  x: Position,
  y: Position,
}

#[derive(Clone,Debug)]
struct GridSummary {
  entry_time: Time,
  pending: usize,
  is_active: bool,
  counts: Vec<usize>,
  done: [HashSet<Coordinate>; 2],
}

impl GridSummary {
  fn init(entry_time: Time) -> Self {
    GridSummary{ entry_time, pending: 0, is_active: true, counts: Vec::new(),
      done: [(); 2].map(|_| HashSet::new()) }
  }

  fn add_count(&mut self, count: usize) {
    self.counts.push(count)
  }

  fn count_squares(&self, time: Time) -> usize {
    if time < self.entry_time {
      return 0
    }
    let time = (time - self.entry_time) as usize;
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

/// How does each the entry time repeat in a direction?
#[derive(Clone,Debug,Default)]
struct Repetition {
  /// The first position where the pattern repeats.
  start: Position,
  /// The amount of time between each grid position in this direction.
  stride: Time,
}

impl Repetition {
  fn update(&mut self, position: Position, delta: Time) -> bool {
    if delta == self.stride {
      return true
    }
    self.start = position;
    self.stride = delta;
    false
  }
}

enum Direction {
  North,
  West,
  South,
  East,
}

#[derive(Clone,Debug,Default)]
struct RepetitionFinder {
  directions: [Repetition; 4],
  previous: [Time; 4],
  done: [bool; 4],
}

impl RepetitionFinder {
  fn find_direction(grid: GridCoordinate) -> Option<(Direction,Position)> {
    match grid.x.cmp(&0) {
      Ordering::Less => Some((Direction::West, grid.x)),
      Ordering::Greater => Some((Direction::East, grid.x)),
      Ordering::Equal => match grid.y.cmp(&0) {
        Ordering::Less => Some((Direction::North, grid.y)),
        Ordering::Equal => None,
        Ordering::Greater => Some((Direction::South, grid.y)),
      }
    }
  }

  fn update(&mut self, grid: GridCoordinate, entry_time: Time) {
    if let Some((dir, pos)) = Self::find_direction(grid) {
      let dir = dir as usize;
      if !self.done[dir] {
        self.done[dir] = self.directions[dir].update(pos, entry_time - self.previous[dir]);
        self.previous[dir] = entry_time;
      }
    }
  }

  fn is_done(&self) -> bool {
    self.done.iter().all(|&x| x)
  }

  fn is_unique(&self, grid: GridCoordinate) -> bool {
    (self.directions[0].start..=self.directions[2].start).contains(&grid.y) &&
        (self.directions[1].start..=self.directions[3].start).contains(&grid.x)
  }
}

#[derive(Clone,Debug)]
pub struct Map {
  spots: Vec<Vec<Spot>>,
  start: Coordinate,
  width: Range<Position>,
  height: Range<Position>,
}

impl Map {
  fn from_str(s: &str) -> Result<Self,String> {
    let mut start : Option<Coordinate> = None;
    let spots = s.lines().enumerate()
        .map(|(y, line)| line.chars().enumerate()
            .map(|(x, ch)| {
              let s = Spot::from_char(ch);
              if let Ok(Spot::Start) = s {
                start = Some(Coordinate {x: x as i32, y: y as i32})
              }
              s })
            .collect::<Result<Vec<Spot>,String>>())
        .collect::<Result<Vec<Vec<Spot>>,String>>()?;
    let start = start.ok_or("No start".to_string())?;
    let width = 0..(spots[0].len() as Position);
    let height = 0..(spots.len() as Position);
    Ok(Map{spots, start, width, height})
  }

  fn contains(&self, location: Coordinate) -> bool {
    self.width.contains(&location.x) && self.height.contains(&location.y)
  }

  fn get_spot(&self, location: Coordinate) -> Spot {
    self.spots[location.y.rem_euclid(self.height.end) as usize]
        [location.x.rem_euclid(self.width.end) as usize]
  }

  fn convert_to_grid(&self, location: Coordinate) -> GridCoordinate {
    GridCoordinate {x: location.x.div_euclid(self.width.end), y: location.y.div_euclid(self.height.end)}
  }

  fn next<const LIMITLESS: bool>(&self, spot: Coordinate) -> SmallVec<[Coordinate;4]> {
    let mut result = SmallVec::new();
    for dir in [Coordinate {x:0, y:-1}, Coordinate {x:-1, y:0},
                Coordinate {x:0, y:1}, Coordinate {x:1, y:0}] {
      let new = spot + dir;
      if (LIMITLESS || self.contains(new)) && self.get_spot(new) != Spot::Rock{
        result.push(new);
      }
    }
    result
  }

  fn moves<const LIMITLESS: bool>(&self, dist: Time) -> usize {
    let mut frontier : HashSet<Coordinate> = HashSet::new();
    let mut done = [(); 2].map(|_| HashSet::new());
    frontier.insert(self.start.clone());
    for t in 0..dist {
      let mut next = HashSet::new();
      for loc in frontier.into_iter() {
        for n in self.next::<LIMITLESS>(loc) {
          if done[t as usize % 2].insert(n) {
            next.insert(n);
          }
        }
      }
      frontier = next;
    }
    done[(dist as usize + 1) % 2].len()
  }

  fn unbounded_moves(&self, dist: Time) -> usize {
    let mut repetitions = RepetitionFinder::default();
    let mut frontier : HashSet<Coordinate> = HashSet::new();
    let mut summaries = HashMap::new();
    frontier.insert(self.start.clone());
    summaries.insert(GridCoordinate {x:0, y:0}, GridSummary::init(0));
    for t in 0..dist {
      let mut next = HashSet::new();
      for loc in frontier.into_iter() {
        for n in self.next::<true>(loc) {
          let grid = self.convert_to_grid(n);
          // Get the summary for the next grid
          let summary = summaries.entry(grid)
              .or_insert_with(|| {
                if grid.x == 0 || grid.y == 0 {
                  repetitions.update(grid, t);
                }
                GridSummary::init(t)});
          if summary.done[t as usize % 2].insert(n) {
            next.insert(n);
            summary.pending += 1;
          }
        }
      }
      let mut done = repetitions.is_done();
      // Update all of the summaries for time t + 1
      for (grid, summary) in summaries.iter_mut() {
        if summary.is_active || summary.pending > 0 {
          summary.add_count(summary.done[t as usize % 2].len());
          summary.is_active = summary.pending > 0;
          summary.pending = 0;
          if summary.is_active {
            done &= !repetitions.is_unique(*grid);
          }
        }
      }
      if done {
        break
      }
      frontier = next;
    }
    println!("Repetitions: {:?}", repetitions);
    0
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input)
    .unwrap() // panics on error
}

pub fn part1(input: &Map) -> usize {
  input.moves::<true>(64)
}

pub fn part2(_input: &Map) -> usize {
  //input.move(26501365)
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
    assert_eq!(668697, input.unbounded_moves(1000));
    //assert_eq!(50, input.moves::<true>(10));
    //assert_eq!(50, input.grid_move(10));
    //assert_eq!(668697, input.grid_move(1000));
  }
}
