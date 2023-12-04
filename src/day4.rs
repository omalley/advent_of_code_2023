use std::cmp::Ordering;
use crate::day2::Game;

#[derive(Debug)]
pub struct Card {
  id: u32,
  wins: Vec<u32>,
  picks: Vec<u32>,
}

impl Card {
  fn from_str(s: &str) -> Result<Self,String> {
    let (title, contents) = s.split_once(": ").ok_or("Can't parse card")?;
    let id = title.split_whitespace().nth(1).ok_or("Can't parse title")?
        .parse::<u32>().map_err(|_| "Can't parse id")?;
    let (win_str, pick_str) = contents.split_once(" | ")
        .ok_or("Can find wins")?;
    let mut wins = win_str.split_whitespace()
        .map(|w| w.parse::<u32>().map_err(|_| format!("Can't parse number {w}")))
        .collect::<Result<Vec<u32>, String>>()?;
    wins.sort_unstable();
    let mut picks = pick_str.split_whitespace()
        .map(|w| w.parse::<u32>().map_err(|_|format!("Can't parse number {w}")))
        .collect::<Result<Vec<u32>, String>>()?;
    picks.sort_unstable();
    Ok(Card{id, wins, picks})
  }

  fn score(&self) -> i32 {
    let mut w = 0;
    let mut p = 0;
    let mut matches = 0;
    while w < self.wins.len() && p < self.picks.len() {
      match self.wins[w].cmp(&self.picks[p]) {
        Ordering::Less => w += 1,
        Ordering::Equal => {
          matches += 1;
          w += 1;
          p += 1;
        },
        Ordering::Greater => p += 1,
      }
    }
    if matches == 0 {
      0
    } else {
      1 << (matches - 1)
    }
  }
}

pub fn generator(input: &str) -> Vec<Card> {
  input.lines()
      .map(|l| Card::from_str(l)
          .map_err(|e| format!("Can't parse game with error [{e}] in {l}")))
      .collect::<Result<Vec<Card>, String>>()
      .unwrap() // panics on error
}

pub fn part1(cards: &Vec<Card>) -> i32 {
  cards.iter().map(|c| c.score()).sum()
}

pub fn part2(cards: &Vec<Card>) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use crate::day4::{generator, part1, part2};

  const INPUT: &str =
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

  #[test]
  fn test_part1() {
    assert_eq!(13, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(467835, part2(&generator(INPUT)));
  }
}
