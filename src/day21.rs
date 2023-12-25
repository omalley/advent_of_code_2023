use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};
use std::ops::Range;
use priority_queue::PriorityQueue;
use smallvec::{SmallVec, smallvec};

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
type Time = u32;

#[derive(Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
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

  fn next<const LIMITLESS: bool>(&self, spot: Location) -> SmallVec<[Location;4]> {
    let mut result = SmallVec::new();
    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
      let new = Location{x: spot.x + dir.0, y: spot.y + dir.1};
      if (LIMITLESS || (self.width.contains(&new.x) && self.height.contains(&new.y))) &&
          self.spots[new.y.rem_euclid(self.height.end) as usize]
              [new.x.rem_euclid(self.width.end) as usize] != Spot::Rock {
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
          match self.spots[y.rem_euclid(self.height.end) as usize][x.rem_euclid(self.width.end) as usize] {
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
    let mut done = [(); 2].map(|_| HashSet::new());
    frontier.insert(self.start.clone());
    for t in 0..dist {
      let mut next = HashSet::new();
      for loc in frontier.into_iter() {
        for n in self.next::<LIMITLESS>(loc) {
          if done[t as usize % 2].insert(n.clone()) {
            next.insert(n);
          }
        }
      }
      frontier = next;
    }
    //self.print_map(&done[(dist + 1) % 2]);
    done[(dist as usize + 1) % 2].len()
  }

  fn summarize_grid(&self, orig_entries: EntryList) -> GridSummary {
    let mut entries = orig_entries.clone();
    entries.sort_unstable();
    let mut frontier : HashSet<Location> = HashSet::new();
    let mut done = [(); 2].map(|_| HashSet::new());
    let mut outside_done = HashSet::new();
    let mut counts = Vec::new();
    let mut exits: Vec<Exit> = Vec::new();
    while !entries.is_empty() && entries[0].time == 0 {
      let entry = entries.remove(0);
      frontier.insert(entry.location.clone());
      done[0].insert(entry.location);
    }
    counts.push(orig_entries.len() - entries.len());
    for time in 1..Time::MAX {
      while !entries.is_empty() && entries[0].time == time {
        let entry = entries.remove(0);
        frontier.insert(entry.location.clone());
        done[time as usize % 2].insert(entry.location);
      }
      let mut next = HashSet::new();
      for loc in frontier.into_iter() {
        for n in self.next::<true>(loc) {
          if !self.width.contains(&n.x) || !self.height.contains(&n.y) {
            let grid= GridOffset{x: n.x.div_euclid(self.width.end),
              y: n.y.div_euclid(self.height.end)};
            if self.width.contains(&loc.x) && self.height.contains(&loc.y) {
              let location = Location{x: n.x.rem_euclid(self.width.end),
                y: n.y.rem_euclid(self.height.end)};
              if let Some(e) = exits.iter_mut()
                  .find(|&e| e.grid == grid.clone()) {
                e.locations.push(EntrySpot{time: time - e.time, location})
              } else {
                exits.push(Exit{ time, grid,
                  locations: smallvec![EntrySpot{time: 0, location}] })
              }
            }
            if outside_done.insert(n.clone()) {
              next.insert(n);
            }
          } else if done[time as usize % 2].insert(n.clone()) {
            next.insert(n);
          }
        }
      }
      // Get the current number of spots marked
      let current_size = done[time as usize % 2].len();
      // If it is the same as 2 steps ago, we are done.
      if counts.len() >= 2 && counts[counts.len() - 2] == current_size {
        break;
      }
      counts.push(current_size);
      frontier = next;
    }
    for e in exits.iter_mut() {
      e.locations.sort_unstable();
    }
    GridSummary{counts, exits}
  }

  fn grid_move(&self, dist: Time) -> usize {
    let mut pending: PriorityQueue<Exit, Reverse<usize>> = PriorityQueue::new();
    let mut cache = HashMap::new();
    let mut done = HashSet::new();
    let mut count = 0;
    pending.push(Exit{time: 0, grid: GridOffset{x: 0, y: 0}, location: self.start.clone()},
                 Reverse(0));
    while let Some((exit, _)) = pending.pop() {
      if done.len() % 10_000 == 0 {
        println!("t = {}, done = {}", exit.time, done.len());
      }
      //println!("popping {:?}, remaining = {}, cache = {}", exit, pending.len(), cache.len());
      let summary = cache.entry(exit.location.clone())
          .or_insert_with(|| self.summarize_grid(exit.location.clone()));
      //println!("{:?}", summary);
      let new_count = summary.count_squares(dist - exit.time);
      count += new_count;
      //println!("adding {} for {:?} at {}", new_count, exit.grid, exit.time);
      done.insert(exit.grid.clone());
      for next in &summary.exits {
        let time = next.time + exit.time;
        let grid = GridOffset{x: next.grid.x + exit.grid.x, y: next.grid.y + exit.grid.y};
        if time < dist && !done.contains(&grid) {
          pending.push_increase(Exit{time, grid, location: next.location.clone() },
                                Reverse(time));
        }
      }
    }
    count
  }
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct GridOffset {
  x: Coordinate,
  y: Coordinate,
}

#[derive(Clone,Debug,Eq,Hash,Ord,PartialEq,PartialOrd)]
struct EntrySpot {
  time: Time,
  location: Location,
}

type EntryList = SmallVec<[EntrySpot; 2]>;

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct Exit {
  time: Time,
  grid: GridOffset,
  locations: EntryList,
}

#[derive(Clone,Debug)]
struct GridSummary {
  counts: Vec<usize>,
  exits: Vec<Exit>,
}

impl GridSummary {
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

pub fn part2(input: &Map) -> usize {
  input.grid_move(26501365)
}

#[cfg(test)]
mod tests {
  use crate::day21::{Exit, generator, GridOffset, Location};

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
    //assert_eq!(668697, input.moves::<true>(1000));
    assert_eq!(50, input.moves::<true>(10));
    assert_eq!(50, input.grid_move(10));
    //assert_eq!(668697, input.grid_move(1000));
  }

  #[test]
  fn test_summary() {
    let input = generator(INPUT);
    let summary = input.summarize_grid(input.start.clone());
    assert_eq!(16, summary.counts[6]);
    assert_eq!(Exit{time:7, grid: GridOffset{x: -1, y: 0}, location: Location{x: 10, y: 4}},
               summary.exits[0]);
  }
}
