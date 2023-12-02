#[derive(Clone,Debug)]
pub struct Draw {
  red: i32,
  blue: i32,
  green: i32,
}

fn parse_int(s: &str) -> Result<i32, String> {
  s.parse().map_err(|_| format!("Can't parse integer - {s}"))
}

impl Draw {
  fn from_str(s: &str) -> Result<Self, String> {
    let mut red = 0;
    let mut blue = 0;
    let mut green = 0;
    for draw_str in s.split(", ") {
      let (count, color) = draw_str.split_once(' ')
        .ok_or("1 word in term")?;
      match color {
        "red" => red += parse_int(count)?,
        "blue" => blue += parse_int(count)?,
        "green" => green += parse_int(count)?,
        _ => return Err(format!("Unknown color: {color}")),
      }
    }
    Ok(Draw{red, blue, green})
  }
}

#[derive(Clone,Debug)]
pub struct Game {
  id: i32,
  draws: Vec<Draw>,
}

impl Game {
  fn from_str(s: &str) -> Result<Self,String> {
    let (title, draw_string) = s.split_once(": ").ok_or("Can't parse game")?;
    let id = parse_int(title.split_whitespace().nth(1).ok_or("Can't parse title")?)?;
    let draws = draw_string.split("; ")
      .map(Draw::from_str).collect::<Result<Vec<Draw>, String>>()?;
    Ok(Game{id, draws})
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
  input.lines()
    .map(|l| Game::from_str(l)
      .map_err(|e| format!("Can't parse game with error [{e}] in {l}")))
    .collect::<Result<Vec<Game>, String>>()
    .unwrap() // panics on error
}

static MAX_DRAW: Draw = Draw{red: 12, green: 13, blue: 14};

pub fn part1(input: &[Game]) -> i32 {
  input.iter().filter(|g| {
      let max = g.max_rocks();
      max.red <= MAX_DRAW.red && max.green <= MAX_DRAW.green && max.blue <= MAX_DRAW.blue})
    .map(|g| g.id)
    .sum()
}

pub fn part2(input: &[Game]) -> i32 {
  input.iter().map(|g| {
      let max = g.max_rocks();
      max.red * max.blue * max.green})
    .sum()
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
    assert_eq!(2286, part2(&generator(INPUT)));
  }
}
