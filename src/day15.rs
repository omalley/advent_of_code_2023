fn hash(word: &str) -> usize {
  word.trim().chars().fold(0, |acc, ch| ((acc + ch as usize) * 17) % 256)
}

pub fn generator(input: &str) -> Vec<String> {
  input.split(',').map(|s| s.to_string()).collect()
}

pub fn part1(input: &[String]) -> usize {
  input.iter().map(|s| hash(s.as_str())).sum()
}

pub fn part2(_input: &[String]) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use crate::day15::{generator, part1, part2, hash};

  const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

  #[test]
  fn test_part1() {
    assert_eq!(1320, part1(&generator(INPUT)));
  }

  #[test]
  fn test_hash() {
    assert_eq!(52, hash("HASH"));
  }

  #[test]
  fn test_part2() {
    assert_eq!(64, part2(&generator(INPUT)));
  }
}
