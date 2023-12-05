use itertools::Itertools;
use std::ops::Range;

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

  fn source_end(&self) -> i64 {
    self.source + self.length
  }

  fn offset(&self) -> i64 {
    self.destination - self.source
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

  fn translate(&self, val: i64) -> i64 {
    self.rules.iter().filter_map(|r| r.apply(val)).next().unwrap_or(val)
  }

  fn translate_ranges(&self, ranges: &[Range<i64>]) -> Vec<Range<i64>> {
    let mut result = Vec::new();
    for rng in ranges {
      let mut current_rule = 0;
      let mut current_val = rng.start;
      while current_val < rng.end {
        // Look through the rules until we find one that may be active
        while current_rule < self.rules.len() &&
            self.rules[current_rule].source_end() <= current_val {
          current_rule += 1;
        }
        if current_rule == self.rules.len() {
          result.push(current_val..rng.end);
          current_val = rng.end;
        } else if current_val < self.rules[current_rule].source {
          let new_end = rng.end.min(self.rules[current_rule].source);
          result.push(current_val..new_end);
          current_val = new_end;
        } else {
          let new_end = rng.end.min(self.rules[current_rule].source_end());
          let offset = self.rules[current_rule].offset();
          result.push(current_val+offset..new_end+offset);
          current_val = new_end;
        }
      }
    }
    result
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
    let seeds: Vec<i64> = seed_list.split_whitespace()
        .map(|w| w.parse::<i64>().map_err(|_| format!("Can't parse integer {w}")))
        .collect::<Result<Vec<i64>,String>>()?;
    Ok(Almanac{seeds, translations: itr.map(KindTranslation::from_str)
        .collect::<Result<Vec<KindTranslation>,String>>()?})
  }

  fn translate(&self, seed: i64) -> i64 {
    let mut val = seed;
    for tr in &self.translations {
      val = tr.translate(val);
    }
    val
  }

  fn translate_ranges(&self, seed: &[Range<i64>]) -> Vec<Range<i64>> {
    let mut val = seed.to_vec();
    for tr in &self.translations {
      val = tr.translate_ranges(&val);
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
  let seed_ranges = almanac.seeds.iter().tuples::<(_,_)>()
      .map(|(start,len)| *start..*start+*len).collect::<Vec<Range<i64>>>();
  almanac.translate_ranges(&seed_ranges).iter().map(|r| r.start).min().unwrap()
}

#[cfg(test)]
mod tests {
  use crate::day5::{generator, KindTranslation, part1, part2};

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
  fn test_translation() {
    let trans =
        KindTranslation::from_str("foo map:\n100 1000 50\n200 2000 25").unwrap();
    assert_eq!(vec![1..1000, 100..150, 1050..2000, 200..225, 2025..3000],
               trans.translate_ranges(&[1..3000]));
    assert_eq!(vec![103..105],
               trans.translate_ranges(&[1003..1005]));
    assert_eq!(vec![4000..4011],
               trans.translate_ranges(&[4000..4011]));
  }

  #[test]
  fn test_part2() {
    assert_eq!(46, part2(&generator(INPUT)));
  }
}
