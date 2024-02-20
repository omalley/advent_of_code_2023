use std::time;
use omalley_aoc2023::{FUNCS, NAMES, utils};

use argh::FromArgs;
use colored::Colorize;
use serde::{Deserialize,Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;

#[derive(FromArgs)]
/** Solution for Advent of Code (https://adventofcode.com/)*/
struct Args {
  /// the input directory
  #[argh(option, short='i', default="String::from(\"input\")")]
  input: String,

  /// days to execute (defaults to all)
  #[argh(positional)]
  days: Vec<usize>,
}

#[derive(Default,Deserialize,Serialize)]
struct Answers {
  // map from day name to answers
  days: BTreeMap<String,Vec<String>>,
}

impl Answers {
  fn make_filename(directory: &str) -> String {
    Path::new(directory).join("answers.yml").to_string_lossy().to_string()
  }

  fn read(directory: &str) -> Self {
    if let Ok(f) = File::open(Self::make_filename(directory)) {
      serde_yaml::from_reader(f).expect("Could not read answers")
    } else {
      Self::default()
    }
  }

  fn update(&mut self, delta_list: &Vec<utils::DayResult>) {
    for delta in delta_list {
      let new_val = delta.get_answers();
      if let Some(prev) =
          self.days.insert(delta.day.to_string(), new_val.clone()) {
        if prev != new_val {
          println!("{}", format!("Output for {} changed from {:?} to {:?}!",
                                 delta.pretty_day(), prev, new_val).bold());
        }
      }
    }
  }

  fn write(&self, directory: &str) {
    let f = std::fs::OpenOptions::new()
      .write(true)
      .create(true)
      .truncate(true)
      .open(Self::make_filename(directory))
      .expect("Couldn't open file");
    serde_yaml::to_writer(f, self).unwrap();
  }
}

fn main() {
  let args: Args = argh::from_env();
  // Which days did the user pick to run?
  let mut day_filter = [args.days.is_empty(); NAMES.len()];
  for day in args.days {
    let name = format!("day{day}");
    if let Some(idx) = NAMES.iter().position(|&n| n == name) {
      day_filter[idx] = true;
    } else {
      panic!("Can't find implementation for {name}.")
    }
  }
  // Read the inputs from the given directory
  println!("{} {}\n", "Reading from".bold(), &args.input);
  let inputs = utils::read_inputs(&args.input, NAMES, &day_filter)
      .expect("Can't read input");

  let results=
    FUNCS.iter().enumerate()
        .filter(|(p, _)| day_filter[*p])
        .map(|(p, f)| {
          let result = f(&inputs[p]);
          println!("{result}");
          result})
        .collect::<Vec<utils::DayResult>>();
  let elapsed = results.iter()
      .map(|r| r.generate_time + r.part1.0 + r.part2.0)
      .sum::<time::Duration>();
  println!("{} {}", "Overall runtime".bold(), format!("({:.2?})", elapsed).dimmed());

  let mut old_answers = Answers::read(&args.input);
  old_answers.update(&results);
  old_answers.write(&args.input);
}
