fn hash(word: &str) -> usize {
  word.chars().fold(0, |acc, ch| ((acc + ch as usize) * 17) % 256)
}

pub fn generator(input: &str) -> Vec<String> {
  input.split(',').map(|s| s.trim().to_string()).collect()
}

pub fn part1(input: &[String]) -> usize {
  input.iter().map(|s| hash(s)).sum()
}

#[derive(Debug,Default)]
struct Lens {
  name: String,
  focal: usize,
}

#[derive(Debug,Default)]
struct Box {
  lens: Vec<Lens>,
}

impl Box {
  fn remove_lens(&mut self, name: &str) {
    self.lens.retain(|l| l.name != name);
  }

  fn add_lens(&mut self, name: &str, focal: usize) {
    for lens in self.lens.iter_mut() {
      if lens.name == name {
        lens.focal = focal;
        return;
      }
    }
    self.lens.push(Lens{name: name.to_string(), focal})
  }

  fn focusing_power(&self, box_id: usize) -> usize {
    self.lens.iter().enumerate()
        .map(|(id, l)| (box_id + 1) * (id + 1) * l.focal)
        .sum()
  }
}
const BOXES_COUNT: usize = 256;

pub fn part2(input: &[String]) -> usize {
  let mut boxes = [(); BOXES_COUNT].map(|_| Box::default());
  for cmd in input {
    if let Some((name, lens)) = cmd.split_once('=') {
      let focal = lens.parse::<usize>().unwrap();
      boxes[hash(name)].add_lens(name, focal);
    } else if let Some((name, _)) = cmd.split_once('-') {
      boxes[hash(name)].remove_lens(name);
    } else {
      panic!("Can't understand command {cmd}");
    }
  }
  boxes.iter().enumerate().map(|(id,b)| b.focusing_power(id)).sum()
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
    assert_eq!(145, part2(&generator(INPUT)));
  }
}
