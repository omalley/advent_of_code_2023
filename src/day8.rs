use std::collections::HashMap;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Direction {
  Left, Right,
}

impl Direction {
  fn from_char(ch: char) -> Result<Direction, String> {
    match ch {
      'L' => Ok(Direction::Left),
      'R' => Ok(Direction::Right),
      _ => Err(format!("Unknown direction - {ch}")),
    }
  }

  fn from_str(s: &str) -> Result<Vec<Direction>, String> {
    s.chars().map(Direction::from_char).collect()
  }
}

#[derive(Clone,Debug)]
pub struct Location {
  name: String,
  left: usize,
  right: usize,
}

impl Location {
  fn from_str(s: &str, location_map: &HashMap<String, usize>) -> Result<Self, String> {
    let (name, targets) = s.split_once(" = ")
        .ok_or("Can't find divider in {s}")?;
    let (left, right) = targets[1..targets.len() -1].split_once(", ")
        .ok_or("Can't split destinations in {s}")?;
    Ok(Location{name: name.to_string(), left: *location_map.get(left).unwrap(),
      right: *location_map.get(right).unwrap()})
  }
}

#[derive(Clone,Debug)]
pub struct Map {
  start: usize,
  goal: usize,
  directions: Vec<Direction>,
  places: Vec<Location>,
}

impl Map {
  const START_LOCATION: &'static str = "AAA";
  const GOAL_LOCATION: &'static str = "ZZZ";

  fn from_str(s: &str) -> Result<Self, String> {
    let (dir_str, room_str) = s.split_once("\n\n")
        .ok_or("Can't find locations!")?;
    let directions = Direction::from_str(dir_str)?;
    let mut location_map: HashMap<String, usize> = HashMap::new();
    for (i, l) in room_str.lines().enumerate() {
      let (id, _) = l.split_once(" = ").ok_or(format!("Can't get name of {l}"))?;
      location_map.insert(id.to_string(), i);
    }
    let places = room_str.lines()
        .map(|l| Location::from_str(l, &location_map))
        .collect::<Result<Vec<Location>,String>>()?;
    Ok(Map{start: *location_map.get(Self::START_LOCATION)
             .ok_or(format!("Can't find {}", Self::START_LOCATION))?,
           goal: *location_map.get(Self::GOAL_LOCATION)
             .ok_or(format!("Can't find {}", Self::GOAL_LOCATION))?,
           directions, places})
  }

  fn step(&self, start: usize, direction: Direction) -> usize {
    let loc = self.places.get(start).unwrap();
    match direction {
      Direction::Left => loc.left,
      Direction::Right => loc.right,
    }
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input).unwrap() // panics on error
}

pub fn part1(input: &Map) -> u64 {
  let mut location = input.start;
  let mut steps = 0;
  for dir in input.directions.iter().cloned().cycle() {
    if location == input.goal {
      break;
    }
    location = input.step(location, dir);
    steps += 1;
  }
  steps
}

pub fn part2(input: &Map) -> u64 {
  0
}

#[cfg(test)]
mod tests {
  use crate::day8::{generator, part1, part2};

  const INPUT: &str =
"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

  #[test]
  fn test_part1() {
    assert_eq!(2, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(0, part2(&generator(INPUT)));
  }
}
