use std::cmp;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::ops::Range;
use array2d::Array2D;

type Position = i32;

#[derive(Clone,Debug,Eq,PartialEq)]
pub struct Block {
  z: Range<Position>,
  x: Range<Position>,
  y: Range<Position>,
}

impl Block {
  fn parse_coordinate(word: &str) -> Result<Vec<Position>,String> {
    word.split(',')
        .map(|w| w.trim().parse::<Position>()
            .map_err(|_| format!("Can't parse {w}")))
        .collect::<Result<Vec<Position>,String>>()
  }

  fn make_range(p1: Position, p2: Position) -> Range<Position> {
    let left = p1.min(p2);
    let right = p1.max(p2);
    left..(right+1)
  }

  fn from_str(line: &str) -> Result<Self,String> {
    let (left,right) = line.split_once('~').ok_or("Can't find ~")?;
    let left_parts = Self::parse_coordinate(left)?;
    let right_parts = Self::parse_coordinate(right)?;
    if left_parts.len() != 3 || right_parts.len() != 3 {
      return Err("Need three positions".to_string());
    }
    let ranges = left_parts.iter().zip(right_parts.iter())
        .map(|(l, r) | Self::make_range(*l, *r))
        .collect::<Vec<Range<Position>>>();
    Ok(Block{z: ranges[2].clone(), x: ranges[0].clone(), y: ranges[1].clone()})
  }
}

impl PartialOrd<Self> for Block {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Block {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.z.start.cmp(&other.z.start)
        .then_with(|| self.x.start.cmp(&other.x.start))
        .then_with(|| self.y.start.cmp(&other.y.start))
        .then_with(|| self.z.end.cmp(&other.z.end))
        .then_with(|| self.x.end.cmp(&other.x.end))
        .then_with(|| self.y.end.cmp(&other.y.end))
  }
}

pub fn generator(input: &str) -> Vec<Block> {
  let mut result = input.lines().enumerate()
      .map(|(i, l)| Block::from_str(l).map_err(|e| format!("line {i}: {e}")))
      .collect::<Result<Vec<Block>,String>>().unwrap();
  result.sort_unstable();
  result
}

#[derive(Clone,Default,Debug)]
struct Cell {
  height: usize,
  block: Option<usize>,
}

#[derive(Debug)]
struct Surface {
  x_range: Range<Position>,
  y_range: Range<Position>,
  surface: Array2D<Cell>,
}

impl Surface {
  fn init(input: &[Block]) -> Self {
    let first = input.first().unwrap();
    let mut x_range = first.x.clone();
    let mut y_range = first.y.clone();
    for blk in input[1..].iter() {
      x_range.start = x_range.start.min(blk.x.start);
      x_range.end = x_range.end.max(blk.x.end);
      y_range.start = y_range.start.min(blk.y.start);
      y_range.end = y_range.end.max(blk.y.end);
    }
    let surface = Array2D::filled_with(Cell::default(), x_range.len(), y_range.len());
    Surface{x_range, y_range, surface}
  }

  fn update(&mut self, blk_id: usize, blk: &Block) -> Option<usize> {
    let mut height = 0;
    for x in blk.x.clone() {
      for y in blk.y.clone() {
        let cell = self.surface.get((x - self.x_range.start) as usize,
                                    (y - self.y_range.start) as usize).unwrap();
        height = height.max(cell.height);
      }
    }
    let mut support = None;
    let mut multiple_supports = false;
    for x in blk.x.clone() {
      for y in blk.y.clone() {
        let cell = self.surface.get_mut((x - self.x_range.start) as usize,
                                    (y - self.y_range.start) as usize).unwrap();
        if !multiple_supports && height == cell.height && cell.block.is_some() {
          if support.is_none() {
            support = cell.block;
          } else if support != cell.block {
            multiple_supports = true;
            support = None;
          }
        }
        cell.height = height + blk.z.len();
        cell.block = Some(blk_id);
      }
    }
    support
  }
}

pub fn part1(input: &[Block]) -> usize {
  if input.is_empty() {
    return 0
  }
  let mut surface = Surface::init(input);
  let mut required = HashSet::new();
  for (blk_id, blk) in input.iter().enumerate() {
    if let Some(req) = surface.update(blk_id, blk) {
      required.insert(req);
    }
  }
  input.len() - required.len()
}

pub fn part2(_input: &[Block]) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use crate::day22::{generator,part1};

  const INPUT: &str =
"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

  #[test]
  fn test_part1() {
    let input = generator(INPUT);
    assert_eq!(5, part1(&input));
  }

  #[test]
  fn test_part2() {
    //
  }
}
