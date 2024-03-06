#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum Direction {
  North,
  West,
  South,
  East,
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum FloorType {
  Space,
  Forrest,
  Slope(Direction),
}

impl FloorType {
  fn from_char(ch: char) -> Result<Self,String> {
    Ok(match ch {
      '.' => Self::Space,
      '#' => Self::Forrest,
      '>' => Self::Slope(Direction::East),
      '<' => Self::Slope(Direction::West),
      '^' => Self::Slope(Direction::North),
      'v' => Self::Slope(Direction::South),
      _ => return Err(format!("Can't understand '{ch}'")),
    })
  }
}

type Position = u32;

#[derive(Clone,Debug,Eq,PartialEq)]
struct Coordinate {
  x: Position,
  y: Position,
}

#[derive(Debug)]
pub struct Map {
  start: Coordinate,
  end: Coordinate,
  bounds: Coordinate,
  floor: Vec<Vec<FloorType>>,
}

impl Map {
  fn from_str(input: &str) -> Result<Self,String> {
    let floor: Vec<Vec<FloorType>> = input.lines()
        .map(|l| l.chars().map(FloorType::from_char)
            .collect::<Result<Vec<FloorType>,String>>())
        .collect::<Result<Vec<Vec<FloorType>>,String>>()?;
    if floor.is_empty() {
      return Err("Empty input".to_string());
    }
    let width = floor.iter().map(|x| x.len())
        .max().unwrap();
    let bounds = Coordinate{x : width as Position, y: floor.len() as Position};
    let start = Coordinate{y: 0,
      x: floor.first().unwrap().iter()
          .position(|x| *x == FloorType::Space).unwrap() as Position};
    let end = Coordinate{y: floor.len() as Position - 1,
      x: floor.last().unwrap().iter()
          .position(|s| *s == FloorType::Space).unwrap() as Position};
    Ok(Map{start, end, floor, bounds})
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input).expect("Problem parsing input")
}

pub fn part1(input: &Map) -> usize {
  println!("{:?}", input);
  0
}

pub fn part2(input: &Map) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use crate::day23::{generator,part1,part2};

  const INPUT: &str =
"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(5, part1(&input));
  }

  #[test]
  fn test_part2() {
    let input = generator(INPUT);
    assert_eq!(7, part2(&input));
  }
}
