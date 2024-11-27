use std::fmt::{Debug, Display, Formatter};
use smallvec::SmallVec;
use strum_macros::EnumIter;
use strum::IntoEnumIterator;

#[derive(Clone,Copy,Debug,EnumIter,Eq,PartialEq)]
pub enum Direction {
  North,
  West,
  South,
  East,
}

impl Direction {
  fn opposite(&self) -> Self {
    match self {
      Direction::North => Direction::South,
      Direction::West => Direction::East,
      Direction::South => Direction::North,
      Direction::East => Direction::West,
    }
  }
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

  fn to_char(&self) -> char {
    match self {
      FloorType::Space => '.',
      FloorType::Forrest => '#',
      FloorType::Slope(dir) =>
      match dir {
        Direction::North => '^',
        Direction::West => '<',
        Direction::South => 'v',
        Direction::East => '>',
      }
    }
  }
}

type Position = i32;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
struct Coordinate {
  x: Position,
  y: Position,
}

#[derive(Clone,Copy,Debug)]
struct DirectedCoordinate {
  coordinate: Coordinate,
  heading: Direction,
}

type NeighborList = SmallVec<[DirectedCoordinate;4]>;

#[derive(Debug)]
pub struct Map {
  start: Coordinate,
  end: Coordinate,
  bounds: Coordinate, // width & height
  floor: Vec<Vec<FloorType>>, // row major layout
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
    let start = Self::find(&floor, "start", 0, FloorType::Space)?;
    let end = Self::find(&floor, "end", bounds.y - 1, FloorType::Space)?;
    Ok(Map{start, end, floor, bounds})
  }

  #[inline]
  fn spot(&self, coordinate: &Coordinate) -> FloorType {
    self.floor[coordinate.y as usize][coordinate.x as usize]
  }

  /// Move in the given direction from the coordinate. Returns None if the result
  /// is out of bounds.
  fn move_to(&self, coordinate: &Coordinate, direction: Direction) -> Option<Coordinate> {
    let mut x = coordinate.x;
    let mut y = coordinate.y;
    match direction {
      Direction::North => y -= 1,
      Direction::West => x -= 1,
      Direction::South => y += 1,
      Direction::East => x += 1,
    }
    if (0..self.bounds.x).contains(&x) && (0..self.bounds.y).contains(&y) {
      Some(Coordinate{x,y})
    } else {
      None
    }
  }

  /// Does this coordinate have more than two neighbors?
  fn is_junction(&self, coordinate: &Coordinate) -> bool {
    if self.spot(coordinate) != FloorType::Forrest {
      // Is it on the edge of the map?
      if coordinate.x == 0 || coordinate.y == 0 ||
        coordinate.x == self.bounds.x - 1 || coordinate.y == self.bounds.y - 1 {
        return true
      }
      let mut count = 0;
      for heading in Direction::iter() {
        if let Some(next) = self.move_to(coordinate, heading) {
          if self.spot(&next) != FloorType::Forrest {
            count += 1;
            if count > 2 {
              return true
            }
          }
        }
      }
    }
    false
  }

  /// Find the given floor tile in a row.
  fn find(floor: &[Vec<FloorType>], name: &str, y: Position, goal: FloorType)
      -> Result<Coordinate, String> {
    floor[y as usize].iter().position(|s| *s == goal)
        .ok_or(format!("{name} not found"))
        .map(|p| Coordinate{x: p as Position, y})
  }

  /// Find the neighbors that we can move to from the given coordinate.
  fn neighbors(&self, spot: &Coordinate) -> NeighborList {
    let mut result = SmallVec::new();
    match self.spot(spot) {
      FloorType::Space => {
        for heading in Direction::iter() {
          if let Some(next) = self.move_to(spot, heading) {
            if self.spot(&next) != FloorType::Forrest {
              result.push(DirectedCoordinate { coordinate: next, heading });
            }
          }
        }
      }
      FloorType::Slope(heading) => {
        let coordinate= self.move_to(spot, heading).unwrap();
        result.push(DirectedCoordinate{coordinate, heading});
      }
      _ => {}
    }
    result
  }

  /// Assuming we aren't at a junction, what is the next location? At dead ends will
  /// return None.
  fn follow(&self, spot: &DirectedCoordinate) -> Option<DirectedCoordinate> {
    let backwards = spot.heading.opposite();
    match self.spot(&spot.coordinate) {
      FloorType::Space => {
        for heading in Direction::iter() {
          if heading != backwards {
            if let Some(next) = self.move_to(&spot.coordinate, heading) {
              if self.spot(&next) != FloorType::Forrest {
                return Some(DirectedCoordinate { coordinate: next, heading });
              }
            }
          }
        }
      }
      FloorType::Slope(heading) if heading != backwards => {
        if let Some(coordinate) = self.move_to(&spot.coordinate, heading) {
          return Some(DirectedCoordinate{coordinate, heading});
        }
      }
      _ => {},
    }
    None
  }
}

type NodeId = u32;

struct JunctionMap<'a> {
  map: &'a Map,
  // for each location, is there a junction there
  locations: Vec<Vec<Option<NodeId>>>,
  // first junction is start, last is end
  junctions: Vec<Coordinate>,
}

impl<'a> JunctionMap<'a> {
  fn from(map: &'a Map) -> Self {
    let mut result = JunctionMap{
      map,
      locations: vec![vec![None; map.bounds.x as usize];
                map.bounds.y as usize],
      junctions: Vec::new(),
    };
    result.add(map.start);
    for y in 1..map.bounds.y-1 {
      for x in 1..map.bounds.x-1 {
        if map.is_junction(&Coordinate{x,y}) {
          result.add(Coordinate{x,y});
        }
      }
    }
    result.add(map.end);
    result
  }

  fn add(&mut self, coordinate: Coordinate) {
    let id = self.junctions.len() as NodeId;
    self.locations[coordinate.y as usize][coordinate.x as usize] = Some(id);
    self.junctions.push(coordinate);
  }

  #[inline]
  fn get(&self, coordinate: Coordinate) -> Option<NodeId> {
    self.locations[coordinate.y as usize][coordinate.x as usize]
  }
}

impl<'a> Display for JunctionMap<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for y in 0..self.map.bounds.y {
      for x in 0..self.map.bounds.x {
        match self.locations[y as usize][x as usize] {
          Some(id) => write!(f, " {:<2}", id)?,
          None => write!(f, " {:<2}", self.map.spot(&Coordinate{x, y}).to_char())?,
        }
      }
      writeln!(f)?;
    }
    Ok(())
  }
}

#[derive(Debug)]
struct SummaryEdge {
  destination: NodeId,
  distance: usize,
}

impl SummaryEdge {
  /// Starting with the given spot, find where the path leads.
  /// It either leads to a junction or dead ends.
  fn from(junctions: &JunctionMap, spot: &DirectedCoordinate) -> Option<Self> {
    let mut distance = 1;
    let mut current = *spot;
    loop {
      if let Some(destination) = junctions.get(current.coordinate) {
        return Some(SummaryEdge{destination, distance});
      }
      if let Some(next) = junctions.map.follow(&current) {
        current = next;
      } else {
        return None
      }
      distance += 1;
    }
  }
}

#[derive(Debug)]
struct SummaryNode {
  coordinate: Coordinate,
  outgoing: SmallVec<[SummaryEdge; 4]>,
}

impl SummaryNode {
  fn from(junctions: &JunctionMap, coordinate: &Coordinate) -> Self {
    let outgoing = junctions.map.neighbors(coordinate).iter()
        .filter_map(|c| SummaryEdge::from(junctions, c)).collect();
    SummaryNode{coordinate: *coordinate, outgoing}
  }
}

#[derive(Debug)]
struct SummaryGraph {
  nodes: Vec<SummaryNode>,
}

impl SummaryGraph {
  fn from(map: &Map) -> Self {
    let junctions = JunctionMap::from(map);
    let nodes = junctions.junctions.iter()
        .map(|junction| SummaryNode::from(&junctions, junction)).collect();
    SummaryGraph{nodes}
  }

  fn max(&self, start: NodeId, distance: usize, used: &mut [bool]) -> usize {
    //println!("visiting {start} at {distance}");
    let mut result = distance;
    used[start as usize] = true;
    for edge in &self.nodes[start as usize].outgoing {
      if !used[edge.destination as usize] {
        result = result.max(self.max(edge.destination, distance + edge.distance, used));
      }
    }
    used[start as usize] = false;
    //println!("result = {result}");
    result
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input)
      .expect("Problem parsing input")
}

pub fn part1(input: &Map) -> usize {
  let graph = SummaryGraph::from(input);
  //println!("graph: {:?}", graph);
  graph.max(0, 0, &mut vec![false; graph.nodes.len()])
}

pub fn part2(_input: &Map) -> usize {
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
    assert_eq!(94, part1(&input));
  }

  #[test]
  fn test_part2() {
    let input = generator(INPUT);
    assert_eq!(7, part2(&input));
  }
}
