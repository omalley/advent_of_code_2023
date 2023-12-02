#[derive(Clone,Debug)]
pub struct Draw {
  red: i32,
  blue: i32,
  green: i32,
}

impl Draw {
  fn from_str(s: &str) -> Self {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for draw_str in s.split(',').map(|s| s.trim()) {
      let (count, color) = draw_str.split_once(' ').unwrap();
      match color {
        "red" => red += count.parse::<i32>().unwrap(),
        "blue" => blue += count.parse::<i32>().unwrap(),
        "green" => green += count.parse::<i32>().unwrap(),
        _ => {},
      }
    }
    Draw{red, blue, green}
  }
}

#[derive(Clone,Debug)]
pub struct Game {
  id: i32,
  draws: Vec<Draw>,
}

impl Game {
  fn from_str(s: &str) -> Self {
    let (title, draw_string) = s.split_once(':').unwrap();
    let id = title.split_whitespace().skip(1).next().unwrap().parse().unwrap();
    let draws = draw_string.split(';').map(Draw::from_str).collect();
    Game{id, draws}
  }

  fn max_rocks(&self) -> Draw {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for d in &self.draws {
      red = red.max(d.red);
      blue = blue.max(d.blue);
      green = green.max(d.green);
    }
    Draw{red, blue, green}
  }
}

pub fn generator(input: &str) -> Vec<Game> {
  input.lines().map(Game::from_str).collect()
}

static MAX_RED: i32 = 12;
static MAX_GREEN: i32 = 13;
static MAX_BLUE: i32 = 14;

pub fn part1(input: &[Game]) -> i32 {
  input.iter().filter(|g| {
      let max = g.max_rocks();
      max.red <= MAX_RED && max.green <= MAX_GREEN && max.blue <= MAX_BLUE})
    .map(|g| g.id)
    .sum()
}

pub fn part2(input: &[Game]) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use crate::day2::{generator, part1, part2};

  const INPUT: &str =
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

  #[test]
  fn test_part1() {
    assert_eq!(8, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    // assert_eq!(142, part2(&generator(INPUT)));
  }
}
