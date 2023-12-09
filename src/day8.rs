use num_integer::Integer;
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
  ends_with_z: bool, // for part2, this location is a goal if the name ends with 'Z'
}

impl Location {
  fn from_str(s: &str, location_map: &HashMap<String, usize>) -> Result<Self, String> {
    let (name, targets) = s.split_once(" = ")
        .ok_or("Can't find divider in {s}")?;
    let (left, right) = targets[1..targets.len() -1].split_once(", ")
        .ok_or("Can't split destinations in {s}")?;
    Ok(Location{name: name.to_string(), left: *location_map.get(left).unwrap(),
      right: *location_map.get(right).unwrap(), ends_with_z: name.ends_with('Z')})
  }
}

#[derive(Clone,Debug)]
pub struct Map {
  start: Option<usize>,       // location AAA if it exists
  goal: Option<usize>,        // location ZZZ if it exists
  directions: Vec<Direction>, // the list of directions that must be repeated
  places: Vec<Location>,      // all of the places
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
    Ok(Map{start: location_map.get(Self::START_LOCATION).copied(),
           goal: location_map.get(Self::GOAL_LOCATION).copied(),
           directions, places})
  }

  /// Take a single step from a given location in the given direction.
  fn step(&self, current: usize, direction: Direction) -> usize {
    let loc = &self.places[current];
    match direction {
      Direction::Left => loc.left,
      Direction::Right => loc.right,
    }
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input).unwrap() // panics on error
}

pub fn part1(input: &Map) -> usize {
  let mut location = input.start.unwrap();
  let goal = input.goal.unwrap();
  for (steps, dir) in input.directions.iter().cloned().cycle().enumerate() {
    if location == goal {
      return steps;
    }
    location = input.step(location, dir);
  }
  panic!("infinite iterator");
}

/// A description of a cycle in the map.
#[derive(Debug)]
struct CycleDescription {
  goals: Vec<usize>, // what are all of the goals before the next cycle?
  start: usize,      // when does the cycle start?
  length: usize,     // how long is the cycle
}

impl CycleDescription {
  fn from_map(input: &Map, start: usize) -> Self {
    let mut visited: Vec<Vec<Option<usize>>>
        = vec![vec![None; input.places.len()]; input.directions.len()];
    let mut goals = Vec::new();
    let mut current = start;
    for (step, dir) in input.directions.iter().cloned().cycle().enumerate() {
      let loc = &input.places[current];
      if let Some(loop_start) = visited[step % input.directions.len()][current] {
        return CycleDescription { goals, start: loop_start, length: step - loop_start }
      }
      if loc.ends_with_z {
        goals.push(step);
      }
      visited[step % input.directions.len()][current] = Some(step);
      current = input.step(current, dir);
    }
    panic!("Shouldn't end loop!");
  }

  /// Is this a simple loop where the loop has a single goal at the end of the loop?
  fn is_simple_loop(&self) -> bool {
    self.goals.len() == 1 && *self.goals.first().unwrap() == self.length
  }

  /// Create an iterator to generate all of the times when we are at a goal.
  fn iter(&self) -> GoalIterator {
    let mut pre_cycle_goals: Vec<usize> =
        self.goals.iter().filter(|&g| *g < self.start).copied().collect();
    pre_cycle_goals.reverse();
    let goals = self.goals.iter().filter(|&g| *g >= self.start)
        .copied().collect();
    GoalIterator{pre_cycle_goals, goals, next_index: 0, cycle_length: self.length, offset: 0}
  }
}

/// Define an infinite iterator that generates the times that we are at a goal spot.
#[derive(Debug)]
struct GoalIterator {
  pre_cycle_goals: Vec<usize>,
  goals: Vec<usize>,
  next_index: usize, // the index of the next goal
  cycle_length: usize, // the length of the cycle
  offset: usize, // the offset from t = 0 depending on how many times we've gone around
}

impl Iterator for GoalIterator {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    if let Some(early) = self.pre_cycle_goals.pop() {
      return Some(early)
    }
    if self.next_index >= self.goals.len() {
      self.next_index = 0;
      self.offset += self.cycle_length;
    }
    let result = self.goals[self.next_index] + self.offset;
    self.next_index += 1;
    Some(result)
  }
}

/// Find the first point where all of the iterators agree. Basically, we keep iterating
/// the lower ones until they agree.
fn find_congruence(iterators: &mut [GoalIterator]) -> usize {
  // Get the first value from each iterator
  let mut current: Vec<usize> = (0..iterators.len())
      .map(|i| iterators[i].next().unwrap()).collect();
  // Keep iterating until all of the iterators are the same
  let mut not_same = true;
  while not_same {
    not_same = false;
    // We know that the answer can't be less than the current max
    let next_step = *current.iter().max().unwrap();
    for i in 0..iterators.len() {
      while current[i] < next_step {
        not_same = true;
        current[i] = iterators[i].next().unwrap();
      }
    }
  }
  current[0]
}

pub fn part2(input: &Map) -> usize {
  let locations = input.places.iter().enumerate()
      .filter_map( |(i, p) | if p.name.ends_with('A') { Some(i) } else { None })
      .collect::<Vec<usize>>();
  let cycles : Vec<CycleDescription> = locations.iter()
      .map(|loc| CycleDescription::from_map(input, *loc)).collect();
  if cycles.iter().all(|c| c.is_simple_loop()) {
    cycles.iter().map(|c| c.length)
        .fold(1_usize, |acc, c| acc.lcm(&c))
  } else {
    let mut iters: Vec<GoalIterator> = cycles.iter().map(|c| c.iter()).collect();
    find_congruence(&mut iters)
  }
}

#[cfg(test)]
mod tests {
  use crate::day8::{CycleDescription, generator, part1, part2};

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

  const INPUT2: &str =
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

  #[test]
  fn test_part2() {
    assert_eq!(6, part2(&generator(INPUT2)));
  }

  const INPUT3: &str =
    "LLLRR

11A = (11Z, XXX)
11Z = (11C, 11D)
11C = (11Z, XXX)
11D = (XXX, 12Z)
12Z = (12B, 12C)
12B = (12C, 12Z)
12C = (12Z, 12B)
XXX = (XXX, XXX)";

  #[test]
  fn test_iter() {
    let input = generator(INPUT2);
    let cycle = CycleDescription::from_map(&input, 0);
    assert_eq!(vec![2, 4, 6, 8], cycle.iter().take(4).collect::<Vec<usize>>());
    let cycle = CycleDescription::from_map(&input, 3);
    assert_eq!(vec![3, 6, 9, 12], cycle.iter().take(4).collect::<Vec<usize>>());

    let input = generator(INPUT3);
    let cycle = CycleDescription::from_map(&input, 0);
    assert_eq!(vec!{1, 3, 5, 8, 12, 14, 16, 20, 23, 27, 29, 31, 35, 38},
               cycle.iter().take(14).collect::<Vec<usize>>());
  }
}
