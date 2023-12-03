// This program is fundamentally about finding the numbers that are
// next to specific symbols in the grid.
// I broke my solution into two parts:
// * Find all of the locations that are adjacent to the symbols.
// * Process all of the numbers, determining their value and proximity
//   to symbols.
use smallvec::SmallVec;

#[derive(Debug)]
pub struct Board {
  field: Vec<Vec<char>>,
  width: usize,
  height: usize,
}

impl Board {
  fn from_str(s: &str) -> Self {
    let field: Vec<Vec<char>> = s.lines()
        .map(|l| l.chars().collect())
        .collect();
    let width = field[0].len();
    let height = field.len();
    Board{field, width, height}
  }
}

pub fn generator(input: &str) -> Board {
  Board::from_str(input)
}

/// A general interface for types that index where the
/// neighbors for symbols are located. Each symbol is
/// given a sequential id.
trait NeighborTracker {
  fn mark(&mut self, x: usize, y:usize, id: usize);
}

/// Mark the board squares that are adjacent to each symbol that satisfies
/// the given filter.
fn find_neighbors<F>(result: &mut dyn NeighborTracker, board: &Board, filter: F)
    where F: Fn(char) -> bool {
  let mut next_id: usize = 0;
  for (y, row) in board.field.iter().enumerate() {
    for (x, spot) in row.iter().enumerate() {
      if filter(*spot) {
        let id = next_id;
        next_id += 1;
        for y_delta in -1i32..=1 {
          for x_delta in -1i32..=1 {
            if (0..board.height as i32).contains(&(y as i32 + y_delta)) &&
                (0..board.width as i32).contains(&(x as i32 + x_delta)) {
              result.mark((x as i32 + x_delta) as usize, (y as i32 + y_delta) as usize, id);
            }
          }
        }
      }
    }
  }
}

/// For part 1, we track whether a given location is next to
/// a symbol.
struct SymbolNeighbors {
  is_neighbor: Vec<Vec<bool>>,
}

impl NeighborTracker for SymbolNeighbors {
  fn mark(&mut self, x: usize, y: usize, _: usize) {
    self.is_neighbor[y][x] = true;
  }
}

/// Generic interface for scanning over the board and categorizing
/// the locations as digits or other.
trait NumberProcessor {
  fn add_digit(&mut self, digit: i32, x: usize, y: usize);
  fn end_number(&mut self);
}

fn process_board(processor: &mut dyn NumberProcessor, board: &Board) {
  // Go through the board by row
  for (y, row) in board.field.iter().enumerate() {
    let mut in_number = false;
    for (x, spot) in row.iter().enumerate() {
      if spot.is_ascii_digit() {
        in_number = true;
        processor.add_digit(*spot as i32 - '0' as i32, x, y);
      } else if in_number {
        processor.end_number();
        in_number = false;
      }
    }
    if in_number {
      processor.end_number();
    }
  }
}

/// For part1, we need to decide which numbers are next to symbols
/// and add them into the result.
struct PartCounter {
  symbol_neighbors: SymbolNeighbors,
  current: i32,
  include_current: bool,
  result: i32,
}

impl NumberProcessor for PartCounter {
  fn add_digit(&mut self, digit: i32, x: usize, y: usize) {
    // for a string of digits, just one location has to be next to a symbol
    self.include_current = self.include_current || self.symbol_neighbors.is_neighbor[y][x];
    // keep the current value of the number
    self.current = self.current * 10 + digit;
  }

  fn end_number(&mut self) {
    // If we have finished a number and it should be included, update the result.
    if self.include_current {
      self.result += self.current;
    }
    self.include_current = false;
    self.current = 0;
  }
}

/// Find the sum of the numbers that are adjacent to symbols.
pub fn part1(board: &Board) -> i32 {
  let mut symbol_neighbors = SymbolNeighbors{is_neighbor: vec![vec![false; board.width]; board.height]};
  find_neighbors(&mut symbol_neighbors, board, |ch| ch != '.' && ch.is_ascii_punctuation());
  let mut processor = PartCounter{symbol_neighbors, current: 0, include_current: false, result: 0};
  process_board(&mut processor, board);
  processor.result
}

/// For each location on the board, find the list of gear ids that are adjacent
/// to it. A given location may be adjacent to multiple gears, unfortunately.
/// We also return the total number of gears found.
#[derive(Debug)]
struct GearMap {
  neighbors: Vec<Vec<SmallVec<[usize; 2]>>>,
  gear_count: usize,
}

impl NeighborTracker for GearMap {
  fn mark(&mut self, x: usize, y: usize, id: usize) {
    self.gear_count = self.gear_count.max(id + 1);
    self.neighbors[y][x].push(id);
  }
}

struct GearCounter {
  gear_map: GearMap,
  /// The current number we are visiting
  current: i32,
  /// All of the gears next to the current number
  current_gears: Vec<usize>,
  /// Indexed by gear id, these are the values next to each gear.
  result: Vec<Vec<i32>>,
}

impl NumberProcessor for GearCounter {
  fn add_digit(&mut self, digit: i32, x: usize, y: usize) {
    // Keep track of the set of all gears this number is next to.
    for new_gear in &self.gear_map.neighbors[y][x] {
      if !self.current_gears.contains(new_gear) {
        self.current_gears.push(*new_gear);
      }
    }
    self.current = self.current * 10 + digit;
  }

  fn end_number(&mut self) {
    // At the end of the number, append it to the list of numbers for each
    // adjacent gear.
    for gear in &self.current_gears {
      self.result[*gear].push(self.current);
    }
    self.current_gears.clear();
    self.current = 0;
  }
}

/// Each '*' that is adjacent to exactly two numbers has a gear ratio that is
/// the product of those two numbers. Return the sum of the gear ratios.
pub fn part2(board: &Board) -> i32 {
  let mut gear_map = GearMap{neighbors: vec![vec![SmallVec::new(); board.width]; board.height],
    gear_count: 0};
  find_neighbors(&mut gear_map, board, |ch| ch == '*');
  let gear_count = gear_map.gear_count;
  let mut processor = GearCounter{gear_map, current:0, current_gears: Vec::new(),
    result: vec![Vec::new(); gear_count]};
  process_board(&mut processor, board);
  processor.result.iter()
      .filter(|g| g.len() == 2)
      .map(|g| g.iter().product::<i32>())
      .sum()
}

#[cfg(test)]
mod tests {
  use crate::day3::{generator, part1, part2};

  const INPUT: &str =
"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

  #[test]
  fn test_part1() {
    assert_eq!(4361, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(467835, part2(&generator(INPUT)));
  }
}
