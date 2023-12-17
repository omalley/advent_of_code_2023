use std::cmp::Reverse;
use std::collections::HashSet;
use priority_queue::PriorityQueue;
use smallvec::SmallVec;

type HeatValue = u32;
type Coordinate = i16;

type Turns = u8;

#[derive(Clone,Debug)]
pub struct Map {
  grid: Vec<Vec<HeatValue>>,
  width: Coordinate,
  height: Coordinate,
}

impl Map {
  fn from_str(input: &str) -> Result<Self,String> {
    let grid = input.lines().map(|line| line.chars()
        .map(|ch| ch.to_digit(10).map(|h| h as HeatValue)
            .ok_or_else(||format!("Can't read number - {ch}")))
          .collect::<Result<Vec<HeatValue>,String>>())
        .collect::<Result<Vec<Vec<HeatValue>>,String>>()?;
    let width = grid[0].len() as Coordinate;
    let height = grid.len() as Coordinate;
    Ok(Map{grid, width, height})
  }

  fn get_cost(&self, x: Coordinate, y: Coordinate) -> Option<HeatValue> {
    if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
      Some(self.grid[y as usize][x as usize])
    } else {
      None
    }
  }

  fn find_minimum<const MIN_TURNS: Turns, const MAX_TURNS: Turns>
                 (&self, start: (Coordinate, Coordinate),
                  finish: (Coordinate, Coordinate)) -> HeatValue {
    let mut done : HashSet<Position<MIN_TURNS,MAX_TURNS>> = HashSet::new();
    let mut pending: PriorityQueue<Position<MIN_TURNS,MAX_TURNS>, Reverse<HeatValue>> =
        PriorityQueue::new();
    pending.push(Position{facing: Direction::Start, straight: MIN_TURNS,
      x: start.0, y: start.1}, Reverse(0));
    while let Some((position, Reverse(heat))) = pending.pop() {
      if position.x == finish.0 && position.y == finish.1 {
        return heat
      }
      for next in position.next() {
        if !done.contains(&next) {
          if let Some(cost) = self.get_cost(next.x, next.y) {
            pending.push_increase(next, Reverse(heat + cost));
          }
        }
      }
      done.insert(position);
    }
    HeatValue::MAX
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input).unwrap()
}

#[derive(Clone,Copy,Debug,Hash,Eq,PartialEq)]
enum Direction {
  East,
  North,
  West,
  South,
  Start,
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct Position<const MIN: Turns, const MAX: Turns> {
  facing: Direction,
  straight: Turns,
  x: Coordinate,
  y: Coordinate,
}

impl<const MIN: Turns, const MAX: Turns> Position<MIN, MAX> {
  fn step(&self, facing: Direction) -> Option<Self> {
    let mut work = self.clone();
    match facing {
      Direction::North => work.y -= 1,
      Direction::East => work.x += 1,
      Direction::South => work.y += 1,
      Direction::West => work.x -= 1,
      _ => panic!("Bad direction {:?}", facing),
    }
    if work.facing == facing {
      work.straight += 1;
      if work.straight > MAX {
        return None
      }
    } else {
      if work.straight < MIN {
        return None
      }
      work.facing = facing;
      work.straight = 1;
    }
    Some(work)
  }

  fn next(&self) -> SmallVec<[Self;3]> {
    // What are the potential directions to move?
    let dirs = match self.facing {
      Direction::East => &[Direction::East, Direction::North, Direction::South],
      Direction::North => &[Direction::North, Direction::East, Direction::West],
      Direction::West => &[Direction::West, Direction::North, Direction::South],
      Direction::South => &[Direction::South, Direction::East, Direction::West],
      Direction::Start => &[Direction::East, Direction::South, Direction::West],
    };
    dirs.iter().filter_map(|d| self.step(*d)).collect()
  }
}

pub fn part1(input: &Map) -> HeatValue {
  input.find_minimum::<0,3>((0, 0), (input.width - 1, input.height - 1))
}

pub fn part2(input: &Map) -> HeatValue {
  input.find_minimum::<4,10>((0, 0), (input.width - 1, input.height - 1))
}

#[cfg(test)]
mod tests {
  use crate::day17::{generator, part1, part2};

  const INPUT: &str =
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";

  #[test]
  fn test_part1() {
    assert_eq!(102, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(94, part2(&generator(INPUT)));
  }
}
