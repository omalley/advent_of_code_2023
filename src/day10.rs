#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Direction {
  North, East, South, West,
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum PipeSection {
  NorthSouth, EastWest, NorthEast, NorthWest, SouthWest, SouthEast, Ground, Start,
}

impl PipeSection {
  fn has_connections(&self) -> [bool; 4] {
    match self {
      PipeSection::NorthSouth => [true, false, true, false],
      PipeSection::EastWest => [false, true, false, true],
      PipeSection::NorthEast => [true, true, false, false],
      PipeSection::NorthWest => [true, false, false, true],
      PipeSection::SouthWest => [false, false, true, true],
      PipeSection::SouthEast => [false, true, true, false],
      PipeSection::Ground => [false, false, false, false],
      PipeSection::Start => [true, true, true, true],
    }
  }

  fn has_direction(&self, dir: Direction) -> bool {
    self.has_connections()[dir as usize]
  }

  fn twist(&self, facing: Direction) -> Option<Direction> {
    match self {
      PipeSection::NorthSouth => {
        match facing {
          Direction::North => Some(Direction::North),
          Direction::South => Some(Direction::South),
          _ => None,
        }
      }
      PipeSection::EastWest => {
        match facing {
          Direction::East => Some(Direction::East),
          Direction::West => Some(Direction::West),
          _ => None,
        }
      }
      PipeSection::NorthEast => {
        match facing {
          Direction::South => Some(Direction::East),
          Direction::West => Some(Direction::North),
          _ => None,
        }
      }
      PipeSection::NorthWest => {
        match facing {
          Direction::South => Some(Direction::West),
          Direction::East => Some(Direction::North),
          _ => None,
        }
      }
      PipeSection::SouthWest => {
        match facing {
          Direction::North => Some(Direction::West),
          Direction::East => Some(Direction::South),
          _ => None,
        }
      }
      PipeSection::SouthEast => {
        match facing {
          Direction::North => Some(Direction::East),
          Direction::West => Some(Direction::South),
          _ => None,
        }
      }
      _ => None,
    }
  }

  fn from_char(ch: char) -> Result<Self, String> {
    match ch {
      '|' => Ok(PipeSection::NorthSouth),
      '-' => Ok(PipeSection::EastWest),
      'L' => Ok(PipeSection::NorthEast),
      'J' => Ok(PipeSection::NorthWest),
      '7' => Ok(PipeSection::SouthWest),
      'F' => Ok(PipeSection::SouthEast),
      '.' => Ok(PipeSection::Ground),
      'S' => Ok(PipeSection::Start),
      _ => Err(format!("Unknown character '{ch}'")),
    }
  }
}

#[derive(Clone,Copy,Debug,Default,PartialEq)]
pub struct Point {
  x: i64,
  y: i64,
}

impl Point {
  fn step(&self, direction: Direction) -> Point {
    match direction {
      Direction::North => Point{x: self.x, y: self.y - 1},
      Direction::South => Point{x: self.x, y: self.y + 1},
      Direction::East => Point{x: self.x + 1, y: self.y},
      Direction::West => Point{x: self.x - 1, y: self.y},
    }
  }
}

#[derive(Clone,Debug)]
struct Walker {
  location: Point,
  facing: Direction,
}

#[derive(Clone,Debug)]
pub struct Map {
  start: Point,
  size: Point,
  grid: Vec<Vec<PipeSection>>,
}

impl Map {
  fn find_start(grid: &[Vec<PipeSection>]) -> Result<Point, String> {
    for (y, row) in grid.iter().enumerate() {
      for (x, loc) in row.iter().enumerate() {
        if *loc == PipeSection::Start {
          return Ok(Point{x: x as i64, y: y as i64})
        }
      }
    }
    Err("Start location not found!".to_string())
  }

  fn from_str(input: &str) -> Result<Self, String> {
    let grid = input.lines().enumerate()
        .map(| (y, l) | l.chars().enumerate()
            .map(| (x, ch) | PipeSection::from_char(ch)
                .map_err(|err| format!("{err} at line {} char {}", y + 1, x + 1)))
            .collect())
        .collect::<Result<Vec<Vec<PipeSection>>, String>>()?;
    let size = Point{x: grid[0].len() as i64, y: grid.len() as i64};
    let start = Self::find_start(&grid)?;
    Ok(Map{start, grid, size})
  }

  fn get_contents(&self, loc: Point) -> Option<PipeSection> {
    if (0..self.size.y).contains(&loc.y) && (0..self.size.x).contains(&loc.x) {
      Some(self.grid[loc.y as usize][loc.x as usize])
    } else {
      None
    }
  }

  fn get_start_walkers(&self) -> Vec<Walker> {
    let mut result = Vec::new();
    if self.get_contents(self.start.step(Direction::North))
        .map(|cont| cont.has_direction(Direction::South)) == Some(true) {
      result.push(Walker{location: self.start, facing: Direction::North});
    }
    if self.get_contents(self.start.step(Direction::East))
        .map(|cont| cont.has_direction(Direction::West)) == Some(true) {
      result.push(Walker{location: self.start, facing: Direction::East});
    }
    if self.get_contents(self.start.step(Direction::South))
        .map(|cont| cont.has_direction(Direction::North)) == Some(true) {
      result.push(Walker{location: self.start, facing: Direction::South})
    }
    if self.get_contents(self.start.step(Direction::West))
        .map(|cont| cont.has_direction(Direction::East)) == Some(true){
      result.push(Walker{location: self.start, facing: Direction::West})
    }
    result
  }

  fn step(&self, walker: &mut Walker) {
    walker.location = walker.location.step(walker.facing);
    walker.facing = self.get_contents(walker.location).unwrap().twist(walker.facing).unwrap();
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input).unwrap() // panics on error
}

pub fn part1(input: &Map) -> usize {
  let mut walkers = input.get_start_walkers();
  let mut distance: usize = 0;
  while distance == 0 || walkers[0].location != walkers[1].location {
    input.step(&mut walkers[0]);
    input.step(&mut walkers[1]);
    distance += 1;
  }
  distance
}

pub fn part2(input: &Map) -> usize {
  let mut walkers = input.get_start_walkers();
  let start_has_north = walkers.iter().any(|w| w.facing == Direction::North);
  let mut pipe_loop =
      vec![vec![false; input.size.x as usize]; input.size.y as usize];
  pipe_loop[input.start.y as usize][input.start.x as usize] = true;
  while walkers[0].location == input.start || walkers[0].location != walkers[1].location {
    for w in walkers.iter_mut() {
      input.step(w);
      pipe_loop[w.location.y as usize][w.location.x as usize] = true;
    }
  }
  let mut inside_count: usize = 0;
  for (y, row) in input.grid.iter().enumerate() {
    let mut wall_count = 0;
    for (x, loc) in row.iter().enumerate() {
      if pipe_loop[y][x] {
        match loc {
          PipeSection::Start => if start_has_north { wall_count += 1 },
          other => if other.has_direction(Direction::North) { wall_count += 1},
        }
      } else if wall_count % 2 == 1 {
        inside_count += 1;
      }
    }
  }
  inside_count
}

#[cfg(test)]
mod tests {
  use crate::day10::{generator, part1, part2};

  const INPUT: &str = "-L|F7\n\
                       7S-7|\n\
                       L|7||\n\
                       -L-J|\n\
                       L|-JF";

  const INPUT2: &str = "7-F7-\n\
                        .FJ|7\n\
                        SJLL7\n\
                        |F--J\n\
                        LJ.LJ";

  #[test]
  fn test_part1() {
    assert_eq!(4, part1(&generator(INPUT)));
    assert_eq!(8, part1(&generator(INPUT2)));
  }

  const INPUT3: &str =
"...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

  const INPUT4: &str =
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

  const INPUT5: &str =
"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

  #[test]
  fn test_part2() {
    assert_eq!(4, part2(&generator(INPUT3)));
    assert_eq!(8, part2(&generator(INPUT4)));
    assert_eq!(10, part2(&generator(INPUT5)));
  }
}
