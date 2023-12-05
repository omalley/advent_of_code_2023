use std::cmp::Ordering;

#[derive(Debug)]
pub struct Card {
  wins: Vec<u32>,
  picks: Vec<u32>,
}

fn read_numbers(s: &str) -> Result<Vec<u32>, String> {
  let mut result = s.split_whitespace()
      .map(|w| w.parse::<u32>().map_err(|_| format!("Can't parse number {w}")))
      .collect::<Result<Vec<u32>, String>>()?;
  result.sort_unstable();
  Ok(result)
}

impl Card {
  fn from_str(s: &str) -> Result<Self,String> {
    let (_, contents) = s.split_once(":").ok_or("Can't find header separator")?;
    let (win_str, pick_str) = contents.split_once("|")
        .ok_or("Can't find wins separator")?;
    Ok(Card{wins: read_numbers(win_str)?, picks: read_numbers(pick_str)?})
  }

  fn matches(&self) -> usize {
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
    matches
  }

  fn score(&self) -> i32 {
    let matches = self.matches();
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

pub fn part1(cards: &[Card]) -> i32 {
  cards.iter().map(|c| c.score()).sum()
}

pub fn part2(cards: &[Card]) -> i32 {
  let mut counts = vec![1; cards.len()];
  for (i, card) in cards.iter().enumerate() {
    let matches = card.matches();
    for winning in i+1..(i+matches+1).min(counts.len()) {
      counts[winning] += counts[i];
    }
  }
  counts.iter().sum()
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
    assert_eq!(30, part2(&generator(INPUT)));
  }
}
