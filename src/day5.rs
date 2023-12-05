#[derive(Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct Rule {
  source: i64,
  length: i64,
  destination: i64,
}

fn parse_int(s: Option<&str>, field_name: &str) -> Result<i64, String> {
  s.ok_or(format!("missing field {field_name}"))?.parse()
      .map_err(|_| format!("Can't parse integer - {}", s.unwrap()))
}
impl Rule {
  fn from_str(s: &str) -> Result<Self,String> {
    let mut word_itr = s.split_whitespace();
    let destination = parse_int(word_itr.next(), "destination")?;
    let source = parse_int(word_itr.next(), "source")?;
    let length = parse_int(word_itr.next(), "length")?;
    Ok(Rule{source, length, destination})
  }

  fn apply(&self, val: i64) -> Option<i64> {
    if (self.source..self.source+self.length).contains(&val) {
      Some(self.destination + val - self.source)
    } else {
      None
    }
  }
}

#[derive(Debug)]
pub struct KindTranslation {
  rules: Vec<Rule>,
}

impl KindTranslation {
  fn from_str(s: &str) -> Result<Self, String> {
    let mut rules = s.lines().skip(1).map(Rule::from_str)
        .collect::<Result<Vec<Rule>, String>>()?;
    rules.sort_unstable();
    Ok(KindTranslation{rules})
  }

  fn apply(&self, val: i64) -> i64 {
    self.rules.iter().filter_map(|r| r.apply(val)).next().unwrap_or(val)
  }
}

#[derive(Debug)]
pub struct Almanac {
  seeds: Vec<i64>,
  translations: Vec<KindTranslation>,
}

impl Almanac {
  fn from_str(s: &str) -> Result<Self, String> {
    let mut itr = s.split("\n\n");
    let (_, seed_list) = itr.next().ok_or("Missing seed deliminator")?
        .split_once(':').ok_or("Missing seed list")?;
    let mut seeds: Vec<i64> = seed_list.split_whitespace()
        .map(|w| w.parse::<i64>().map_err(|_| format!("Can't parse integer {w}")))
        .collect::<Result<Vec<i64>,String>>()?;
    seeds.sort_unstable();
    Ok(Almanac{seeds, translations: itr.map(KindTranslation::from_str)
        .collect::<Result<Vec<KindTranslation>,String>>()?})
  }

  fn translate(&self, seed: i64) -> i64 {
    let mut val = seed;
    for tr in &self.translations {
      val = tr.apply(val);
    }
    val
  }
}

pub fn generator(input: &str) -> Almanac {
  Almanac::from_str(input)
      .unwrap() // panics on error
}

pub fn part1(almanac: &Almanac) -> i64 {
  almanac.seeds.iter().map(|s| almanac.translate(*s)).min().unwrap()
}

pub fn part2(almanac: &Almanac) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use crate::day5::{generator, part1, part2};

  const INPUT: &str =
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

  #[test]
  fn test_part1() {
    assert_eq!(35, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(30, part2(&generator(INPUT)));
  }
}
