#[derive(Clone,Copy,Debug)]
pub enum Mirror {
  Ground,
  ForwardMirror,
  BackwardMirror,
  VerticalSplitter,
  HorizontalSplitter,
}

impl Mirror {
  fn from_char(ch: char) -> Result<Mirror,String> {
    match ch {
      '.' => Ok(Mirror::Ground),
      '/' => Ok(Mirror::ForwardMirror),
      '\\' => Ok(Mirror::BackwardMirror),
      '-' => Ok(Mirror::HorizontalSplitter),
      '|' => Ok(Mirror::VerticalSplitter),
      _ => Err(format!("Unknown character - {ch}")),
    }
  }
}

#[derive(Clone,Debug)]
pub struct Map {
  mirrors: Vec<Vec<Mirror>>,
  width: usize,
  height: usize,
}

impl Map {
  fn from_str(input: &str) -> Result<Self,String> {
    let mirrors = input.lines().map(|line| line.chars().map(Mirror::from_char)
          .collect::<Result<Vec<Mirror>,String>>())
        .collect::<Result<Vec<Vec<Mirror>>,String>>()?;
    let width = mirrors[0].len();
    let height = mirrors.len();
    Ok(Map{mirrors, width, height})
  }

  fn get(&self, light: &Light) -> Option<Mirror> {
    if light.x >= 0 && light.x < self.width as i32 &&
        light.y >= 0 && light.y < self.height as i32 {
      Some(self.mirrors[light.y as usize][light.x as usize])
    } else {
      None
    }
  }

  fn energize(&self) -> usize {
    let mut pending = vec![Light::default()];
    let mut energized = EnergizedMap::new(self.width, self.height);
    while let Some(prev) = pending.pop() {
      // Are we still on the map?
      if let Some(mirror) = self.get(&prev) {
        // If we've already come this direction, stop here.
        if energized.mark(&prev) {
          continue
        }
        // Let the light bounce around
        let mut next = prev.clone();
        if let Some(other) = next.bounce(mirror) {
          let mut other = other.clone();
          other.advance();
          pending.push(other);
        }
        next.advance();
        pending.push(next);
      }
    }
    energized.count()
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input).unwrap()
}

struct EnergizedMap {
  energized: Vec<Vec<[bool; 4]>>,
}

impl EnergizedMap {
  fn new(width: usize, height: usize) -> Self {
    Self{energized: vec![vec![[false; 4]; width]; height]}
  }

  fn mark(&mut self, light: &Light) -> bool {
    let prev = self.energized[light.y as usize][light.x as usize][light.facing as usize];
    self.energized[light.y as usize][light.x as usize][light.facing as usize] = true;
    prev
  }

  fn count(&self) -> usize {
    let mut result = 0;
    for row in &self.energized {
      for spot in row {
        if spot.iter().any(|e| *e) {
          result += 1;
        }
      }
    }
    result
  }

  fn print(&self) {
    for row in &self.energized {
      for spot in row {
        let val: usize = spot.iter().enumerate()
            .map(|(i, b)| (if *b { 1 } else { 0 }) << i).sum();
        print!("{val}");
      }
      println!();
    }
  }
}

#[derive(Clone,Copy,Debug,Default)]
enum Direction {
  #[default]
  East,
  North,
  West,
  South,
}

#[derive(Clone,Debug,Default)]
struct Light {
  facing: Direction,
  x: i32,
  y: i32,
}

impl Light {
  fn advance(&mut self) {
    match self.facing {
      Direction::North => self.y -= 1,
      Direction::East => self.x += 1,
      Direction::West => self.x -= 1,
      Direction::South => self.y += 1,
    }
  }

  fn bounce(&mut self, mirror: Mirror) -> Option<Light> {
    match mirror {
      Mirror::ForwardMirror => {
        match self.facing {
          Direction::North => self.facing = Direction::East,
          Direction::East => self.facing = Direction::North,
          Direction::South => self.facing = Direction::West,
          Direction::West => self.facing = Direction::South,
        }
        None
      },
      Mirror::BackwardMirror => {
        match self.facing {
          Direction::North => self.facing = Direction::West,
          Direction::West => self.facing = Direction::North,
          Direction::South => self.facing = Direction::East,
          Direction::East => self.facing = Direction::South,
        }
        None
      },
      Mirror::HorizontalSplitter => {
        match self.facing {
          Direction::North | Direction::South => {
            self.facing = Direction::West;
            let mut other = self.clone();
            other.facing = Direction::East;
            Some(other)
          },
          _ => None,
        }
      },
      Mirror::VerticalSplitter => {
        match self.facing {
          Direction::East | Direction::West => {
            self.facing = Direction::North;
            let mut other = self.clone();
            other.facing = Direction::South;
            Some(other)
          },
          _ => None,
        }
      },
      Mirror::Ground => None,
    }
  }
}

pub fn part1(input: &Map) -> usize {
  input.energize()
}

pub fn part2(input: &Map) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use crate::day16::{generator, part1, part2};

  const INPUT: &str =
".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";

  #[test]
  fn test_part1() {
    assert_eq!(46, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(145, part2(&generator(INPUT)));
  }
}
