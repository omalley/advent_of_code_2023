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

/// Find the board squares that are adjacent to a symbol.
fn find_symbol_neighbors(board: &Board) -> Vec<Vec<bool>> {
  let mut result = vec![vec![false; board.width]; board.height];
  for (y, row) in board.field.iter().enumerate() {
    for (x, spot) in row.iter().enumerate() {
      if *spot != '.' && spot.is_ascii_punctuation() {
        for y_delta in -1i32..=1 {
          for x_delta in -1i32..=1 {
            if (0..board.height as i32).contains(&(y as i32 + y_delta)) &&
                (0..board.width as i32).contains(&(x as i32 + x_delta)) {
              result[(y as i32 + y_delta) as usize][(x as i32 + x_delta) as usize] = true;
            }
          }
        }
      }
    }
  }
  result
}

/// Find the sum of the numbers that are adjacent to symbols.
pub fn part1(board: &Board) -> i32 {
  let mut result = 0;
  let symbol_neighbors = find_symbol_neighbors(board);
  let mut include_current = false;
  let mut current = 0;
  // Go through the board looking for numbers that are adjacent to symbols.
  for (y, row) in board.field.iter().enumerate() {
    for (x, spot) in row.iter().enumerate() {
      if spot.is_ascii_digit() {
        // for a string of digits, just one location has to be next to a symbol
        include_current = include_current || symbol_neighbors[y][x];
        // keep the current value of the number
        current = current * 10 + *spot as i32 - '0' as i32;
      } else {
        // If we have finished a number and it should be included, update the result.
        if include_current {
          result += current;
        }
        include_current = false;
        current = 0;
      }
    }
  }
  result
}

/// For each location on the board, find the list of gear ids that are adjacent
/// to it. A given location may be adjacent to multiple gears, unfortunately.
/// We also return the total number of gears found.
fn find_gear_neighbors(board: &Board) -> (Vec<Vec<Vec<usize>>>, usize) {
  let mut next_id: usize = 0;
  let mut result = vec![vec![Vec::new(); board.width]; board.height];
  for (y, row) in board.field.iter().enumerate() {
    for (x, spot) in row.iter().enumerate() {
      if *spot == '*' {
        let id = next_id;
        next_id += 1;
        for y_delta in -1i32..=1 {
          for x_delta in -1i32..=1 {
            if (0..board.height as i32).contains(&(y as i32 + y_delta)) &&
                (0..board.width as i32).contains(&(x as i32 + x_delta)) {
              result[(y as i32 + y_delta) as usize][(x as i32 + x_delta) as usize].push(id);
            }
          }
        }
      }
    }
  }
  (result, next_id)
}

/// Each '*' that is adjacent to exactly two numbers has a gear ratio that is
/// the product of those two numbers. Return the sum of the gear ratios.
pub fn part2(board: &Board) -> i32 {
  let (gear_neighbors, gear_count) = find_gear_neighbors(board);
  // Find the numbers next to each gear indexed by id.
  let mut gear_ratios: Vec<Vec<i32>> = vec![Vec::new(); gear_count];
  let mut current_gears: Vec<usize> = Vec::new();
  let mut current = 0;
  for (y, row) in board.field.iter().enumerate() {
    for (x, spot) in row.iter().enumerate() {
      if spot.is_ascii_digit() {
        // Keep track of the set of all gears this number is next to.
        for new_gear in &gear_neighbors[y][x] {
          if !current_gears.contains(new_gear) {
            current_gears.push(*new_gear);
          }
        }
        current = current * 10 + *spot as i32 - '0' as i32;
      } else {
        // At the end of the number, append it to the list of numbers for each
        // adjacent gear.
        if current != 0 {
          for gear in &current_gears {
            gear_ratios[*gear].push(current);
          }
        }
        current_gears.clear();
        current = 0;
      }
    }
  }
  gear_ratios.iter()
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
