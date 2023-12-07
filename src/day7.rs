#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub enum Rank {
  WildJack, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

impl Rank {
  fn from_char(ch: char) -> Result<Rank, String> {
    match ch {
      '2' => Ok(Rank::Two),
      '3' => Ok(Rank::Three),
      '4' => Ok(Rank::Four),
      '5' => Ok(Rank::Five),
      '6' => Ok(Rank::Six),
      '7' => Ok(Rank::Seven),
      '8' => Ok(Rank::Eight),
      '9' => Ok(Rank::Nine),
      'T' => Ok(Rank::Ten),
      'J' => Ok(Rank::Jack),
      'Q' => Ok(Rank::Queen),
      'K' => Ok(Rank::King),
      'A' => Ok(Rank::Ace),
      _ => Err(format!("Unknown rank - {ch}")),
    }
  }

  fn from_str(s: &str) -> Result<Vec<Rank>, String> {
    s.chars().map(Rank::from_char).collect()
  }
}

#[derive(Clone,Copy,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub enum HandKind {
  HighCard, OnePair, TwoPair, ThreeOfAKind, FullHouse, FourOfAKind, FiveOfAKind,
}

const HAND_SIZE: usize = 5;

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct Hand {
  kind: HandKind,
  cards: [Rank; HAND_SIZE],
  bid: u64,
}

impl Hand {
  fn get_kind(cards: &[Rank]) -> Result<HandKind, String> {
    let mut counts = [0; 14];
    for c in cards {
      counts[*c as usize] += 1;
    }
    let wild_cards = counts[Rank::WildJack as usize];
    counts[Rank::WildJack as usize] = 0;
    counts.sort_by(|a, b| b.cmp(a));
    counts[0] += wild_cards;
    match counts[0] {
      1 => Ok(HandKind::HighCard),
      2 => Ok(if counts[1] == 1 { HandKind::OnePair } else { HandKind::TwoPair }),
      3 => Ok(if counts[1] == 1 { HandKind::ThreeOfAKind } else { HandKind::FullHouse }),
      4 => Ok(HandKind::FourOfAKind),
      5 => Ok(HandKind::FiveOfAKind),
      _ => Err(format!("Bad hand kind with {}", counts[0])),
    }
  }

  fn from_str(s: &str) -> Result<Self, String> {
    let mut words = s.split_whitespace();
    let cards: [Rank; HAND_SIZE] = Rank::from_str(words.next()
        .ok_or(format!("Missing cards in {s}"))?)?.try_into().unwrap();
    let bid = words.next().ok_or("Missing bid")?
        .parse::<u64>().map_err(|_| format!("Can't parse bid in {s}"))?;
    let kind = Self::get_kind(&cards)?;
    Ok(Hand { kind, cards, bid })
  }
}

pub fn generator(input: &str) -> Vec<Hand> {
  input.lines().map(Hand::from_str).collect::<Result<Vec<Hand>, String>>()
      .unwrap() // panics on error
}

pub fn part1(input: &[Hand]) -> u64 {
  let mut hands = input.to_vec();
  hands.sort_unstable();
  hands.iter().enumerate().map(|(i, c) | (i as u64 + 1) * c.bid).sum()
}

pub fn part2(input: &[Hand]) -> u64 {
  let mut hands = input.to_vec();
  for h in hands.iter_mut() {
    for c in h.cards.iter_mut() {
      if *c == Rank::Jack {
        *c = Rank::WildJack;
      }
    }
    h.kind = Hand::get_kind(&h.cards).unwrap();
  }
  hands.sort_unstable();
  hands.iter().enumerate().map(|(i, c) | (i as u64 + 1) * c.bid).sum()
}

#[cfg(test)]
mod tests {
  use crate::day7::{generator, part1, part2};

  const INPUT: &str =
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

  #[test]
  fn test_part1() {
    assert_eq!(6440, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(5905, part2(&generator(INPUT)));
  }
}
