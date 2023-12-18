use std::cmp::Ordering;
use std::ops::Range;
use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};

type Coordinate = i32;
type Color = u32;

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  fn from_str(s: &str) -> Result<Self, String> {
    match s {
      "R" => Ok(Direction::Right),
      "L" => Ok(Direction::Left),
      "D" => Ok(Direction::Down),
      "U" => Ok(Direction::Up),
      _ => Err(format!("Unknown direction - {s}")),
    }
  }

  /// The number to separate out the direction from distance in the color.
  const COLOR_DIVISOR: Color = 16;

  fn from_color(c: Color) -> Result<Self,String> {
    match c % Self::COLOR_DIVISOR {
      0 => Ok(Direction::Right),
      1 => Ok(Direction::Down),
      2 => Ok(Direction::Left),
      3 => Ok(Direction::Up),
      _ => Err(format!("Unknown direction {} from {c}", c % Self::COLOR_DIVISOR)),
    }
  }
}

#[derive(Clone,Debug,Default,PartialEq)]
pub struct Position {
  x: Coordinate,
  y: Coordinate,
}

impl Position {
  fn move_to(&mut self, direction: Direction, distance: Coordinate) {
    match direction {
      Direction::Up => self.y -= distance,
      Direction::Right => self.x += distance,
      Direction::Down => self.y += distance,
      Direction::Left => self.x -= distance,
    }
  }
}

#[derive(Clone,Debug)]
pub struct Edge {
  start: Position,
  direction: Direction,
  end: Position,
  color: Color,
}

impl Edge {
  fn from_str(line: &str, start: Position) -> Result<Self,String> {
    let mut words = line.split_whitespace();
    let direction = Direction::from_str(
      words.next().ok_or(format!("Can't find direction in {line}."))?)?;
    let distance = words.next()
        .ok_or(format!("Can't find distance in {line}"))?
        .parse::<Coordinate>().map_err(|_| format!("Can't parse distance in {line}"))?;
    let mut color = words.next().ok_or(format!("Can't find color in {line}"))?;
    color = color.strip_prefix("(#").ok_or("can't remove color prefix".to_string())?;
    color = color.strip_suffix(')').ok_or("can't remove color suffix".to_string())?;
    let color = Color::from_str_radix(color, 16)
        .map_err(|_| format!("Can't parse color - {color}"))?;
    let mut end = start.clone();
    end.move_to(direction, distance);
    Ok(Edge{start: start.clone(), direction, end, color})
  }

  fn from_color(color: Color, start: &Position) -> Result<Self,String> {
    let direction = Direction::from_color(color)?;
    let mut end = start.clone();
    let distance = (color / Direction::COLOR_DIVISOR) as Coordinate;
    end.move_to(direction, distance);
    Ok(Edge{start: start.clone(), direction, end, color: 0})
  }
}

#[derive(Clone,Debug,Eq,PartialEq)]
struct EdgeBox {
  y: Range<Coordinate>,
  x: Range<Coordinate>,
  is_vertical: bool,
}

impl EdgeBox {
  fn x_cmp(&self, other: &Self) -> Ordering {
    match self.x.start.cmp(&other.x.start) {
      Ordering::Equal => self.x.end.cmp(&other.x.end),
      result => result,
    }
  }
}

impl Ord for EdgeBox {
  fn cmp(&self, other: &Self) -> Ordering {
    match self.y.start.cmp(&other.y.start) {
      Ordering::Equal => {
        match self.x.start.cmp(&other.x.start) {
          Ordering::Equal => match self.y.end.cmp(&other.y.end) {
            Ordering::Equal => self.x.end.cmp(&other.x.end),
            result => result,
          }
          result => result,
        }
      }
      result => result,
    }
  }
}

impl PartialOrd for EdgeBox {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

#[derive(Clone,Debug)]
struct EdgeMap {
  edges: Vec<EdgeBox>,
  width: Range<Coordinate>,
  height: Range<Coordinate>,
}

impl EdgeMap {
  fn compute_volume(&self) -> u64 {
    let mut y = self.height.start;
    let mut active: Vec<EdgeBox> = Vec::new();
    let mut next = 0;
    let mut count: u64 = 0;
    while y < self.height.end {
      // update the active edges for the new row
      active.retain(|l| l.y.contains(&y));
      while next < self.edges.len() && self.edges[next].y.contains(&y) {
        active.push(self.edges[next].clone());
        next += 1;
      }
      active.sort_unstable_by(|a,b| a.x_cmp(b));
      let mut wall_count = 0;
      let mut in_row_count = 0;
      let mut x = self.width.start;
      for edge in &active {
        if x < edge.x.start {
          if wall_count % 2 == 1 {
            in_row_count +=  edge.x.start - x;
          }
          x = edge.x.start;
        }
        if x < edge.x.end {
          in_row_count += edge.x.end - x;
          x = edge.x.end;
        }
        if edge.is_vertical {
          wall_count += 1;
        }
      }
      let next_edge = self.edges.get(next).map(|e| e.y.start)
          .unwrap_or(y+1);
      let duplicate_rows = active.iter()
          .map(|e| e.y.end).min().unwrap_or(y + 1).min(next_edge) - y;
      count += duplicate_rows as u64 * in_row_count as u64;
      y += duplicate_rows;
    }
    count
  }
}

#[derive(Clone,Debug)]
pub struct Map {
  edges: Vec<Edge>,
  width: Range<Coordinate>,
  height: Range<Coordinate>,
}

impl Map {
  fn from_str(input: &str) -> Result<Self,String> {
    let mut current = Position::default();
    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;
    let edges = input.lines().map(|l| {
      let e = Edge::from_str(l, current.clone());
      if let Ok(edge) = &e {
        left = left.min(edge.end.x);
        right = right.max(edge.end.x);
        top = top.min(edge.end.y);
        bottom = bottom.max(edge.end.y);
        current = edge.end.clone();
      }
      e
    }).collect::<Result<Vec<Edge>,String>>()?;
    Ok(Map{edges, width: left..right+1, height: top..bottom+1})
  }

  /// Build a map of the edges of the ditches, where each edge is modelled as a bounding
  /// box. Vertical edges do not include the southern end point so that our vertical
  /// wall counting is easier.
  fn build_edge_map(&self) -> EdgeMap {
    let mut edges = Vec::new();
    for e in &self.edges {
      match e.direction {
        Direction::Up =>
          // The verticals walls explicitly don't cover the last square, so that
          // we can count them well.
          edges.push(EdgeBox{x: e.start.x..e.start.x+1,
            y: e.end.y..e.start.y, is_vertical: true}),
        Direction::Down =>
          edges.push(EdgeBox{x: e.start.x..e.start.x+1,
            y: e.start.y..e.end.y, is_vertical: true}),
        Direction::Right =>
          edges.push(EdgeBox{y: e.start.y..e.start.y+1,
            x: e.start.x..e.end.x+1, is_vertical: false}),
        Direction::Left =>
          edges.push(EdgeBox{y: e.start.y..e.start.y+1,
            x: e.end.x..e.start.x+1, is_vertical: false}),
      }
    }
    edges.sort_unstable();
    EdgeMap{edges, width: self.width.clone(), height: self.height.clone()}
  }

  /// Reinterpret the colors on the edges as the instructions to follow.
  fn reinterpret_colors(&self) -> Result<Self, String> {
    let mut current = Position::default();
    let mut left = 0;
    let mut right = 1;
    let mut top = 0;
    let mut bottom = 1;
    let edges = self.edges.iter().map(|original_edge| {
      let e = Edge::from_color(original_edge.color, &current);
      if let Ok(edge) = &e {
        left = left.min(edge.end.x);
        right = right.max(edge.end.x + 1);
        top = top.min(edge.end.y);
        bottom = bottom.max(edge.end.y + 1);
        current = edge.end.clone();
      }
      e
    }).collect::<Result<Vec<Edge>,String>>()?;
    Ok(Map{edges, width: left..right+1, height: top..bottom+1})
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input).unwrap()
}

pub fn part1(input: &Map) -> u64 {
  //save_shape(input, "day18.png");
  input.build_edge_map().compute_volume()
}

pub fn part2(input: &Map) -> u64 {
  input.reinterpret_colors().unwrap().build_edge_map().compute_volume()
}

const BOX_WIDTH: u32 = 11;

fn translate_coord(val: Coordinate) -> f32 {
  val as f32 * BOX_WIDTH as f32 + (BOX_WIDTH / 2) as f32
}

pub fn save_picture(input: &Map, filename: &str) {
  let mut pixmap = Pixmap::new(input.width.len() as u32 * BOX_WIDTH,
                               input.height.len() as u32 * BOX_WIDTH).unwrap();
  let mut path_builder = PathBuilder::new();
  path_builder.move_to(translate_coord(0 - input.width.start),
                       translate_coord(0 - input.height.start));
  for edge in &input.edges {
    path_builder.line_to(translate_coord(edge.end.x - input.width.start),
                         translate_coord(edge.end.y - input.height.start));
  }
  path_builder.close();
  let path = path_builder.finish().unwrap();
  let mut paint = Paint::default();
  paint.set_color_rgba8( 13, 139, 40 , 255);
  pixmap.fill_path(&path, &paint, FillRule::EvenOdd, Transform::default(), None);
  paint.set_color_rgba8(255, 255, 255, 255);
  let stroke = Stroke { width: 3.0, ..Default::default() };
  pixmap.stroke_path(&path, &paint, &stroke, Transform::default(), None);
  pixmap.save_png(filename).unwrap()
}

#[cfg(test)]
mod tests {
  use crate::day18::{generator, part1, part2};

  const INPUT: &str =
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";

  #[test]
  fn test_part1() {
    assert_eq!(62, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(952408144115, part2(&generator(INPUT)));
  }
}
