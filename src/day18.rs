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
  distance: Coordinate,
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
    color = color.strip_suffix(")").ok_or("can't remove color suffix".to_string())?;
    let color = Color::from_str_radix(color, 16)
        .map_err(|_| format!("Can't parse color - {color}"))?;
    let mut end = start.clone();
    end.move_to(direction, distance);
    Ok(Edge{start: start.clone(), direction, distance, end, color})
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

  fn set_line(&self, grid: &mut Vec<Vec<Spot>>, start: &Position,
              end: &Position, direction: Direction, spot: Spot) {
    let mut posn = start.clone();
    grid[(posn.y - self.height.start) as usize][(posn.x - self.width.start) as usize] = spot;
    while posn != *end {
      posn.move_to(direction, 1);
      grid[(posn.y - self.height.start) as usize][(posn.x - self.width.start) as usize] = spot;
    }
  }

  pub fn find_inside(&self) -> (Vec<Vec<Spot>>, usize) {
    let mut result =
        vec![vec![Spot::Outside; self.width.len()]; self.height.len()];
    for edge in &self.edges {
      match edge.direction {
        Direction::Up => {
          self.set_line(&mut result, &edge.start, &edge.end, edge.direction, Spot::Vertical);
          self.set_line(&mut result, &edge.end, &edge.end, edge.direction, Spot::Wall);
        }
        Direction::Down => {
          self.set_line(&mut result, &edge.start, &edge.end, edge.direction, Spot::Vertical);
          self.set_line(&mut result, &edge.start, &edge.start, edge.direction, Spot::Wall);
        }
        _ => if edge.distance > 1 {
          let mut start = edge.start.clone();
          start.move_to(edge.direction, 1);
          let mut end = edge.end.clone();
          end.move_to(edge.direction, -1);
          self.set_line(&mut result, &start, &end, edge.direction, Spot::Wall)
        },
      }
    }
    let mut count = 0;
    for row in result.iter_mut() {
      let mut wall_count = 0;
      for spot in row.iter_mut() {
        match spot {
          Spot::Vertical => {wall_count += 1; count += 1},
          Spot::Outside => if wall_count % 2 == 1 { *spot = Spot::Inside; count += 1 },
          Spot::Wall => count += 1,
          _ => {},
        }
      }
    }
    (result, count)
  }
}

pub fn generator(input: &str) -> Map {
  Map::from_str(input).unwrap()
}

const BOX_WIDTH: u32 = 11;

fn translate_coord(val: Coordinate) -> f32 {
  val as f32 * BOX_WIDTH as f32 + (BOX_WIDTH / 2) as f32
}

pub fn save_shape(input: &Map, filename: &str) {
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

#[derive(Clone,Copy,Debug)]
pub enum Spot {
  Outside,
  Wall,
  Vertical,
  Inside,
}

pub fn part1(input: &Map) -> usize {
  //save_shape(input, "day18.png");
  let (_grid, count) = input.find_inside();
  count
}

pub fn part2(input: &Map) -> usize {
  0
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
