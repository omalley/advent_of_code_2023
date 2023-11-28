use omalley_aoc2023::{FUNCS,NAMES,utils};

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

  /// a single day to execute (defaults to all)
  #[argh(positional)]
  day: Option<usize>,
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
    // Did the user pick a single day to run
    let day_filter: Option<usize> = match args.day {
        Some(day) => {
            let name = format!("day{}", day);
            Some(NAMES.iter().position(|x| **x == name)
              .expect("Requested an unimplemented day"))
        },
        None => None
    };
    // Read the inputs from the given directory
    println!("{} {}\n", "Reading from".bold(), &args.input);
    let inputs = utils::read_inputs(&args.input, NAMES)
      .expect("Can't read input dir");

    let (elapsed, results) = utils::time(&|| {
        FUNCS.iter().enumerate()
          .filter(|(p, _)| day_filter.is_none() || day_filter.unwrap() == *p)
          .map(|(p, f)| f(&inputs[p]))
          .collect::<Vec<utils::DayResult>>()
    });

    for r in &results {
      println!("{}", r);
    }
    println!("{} {}", "Overall runtime".bold(), format!("({:.2?})", elapsed).dimmed());

    let mut old_answers = Answers::read(&args.input);
    old_answers.update(&results);
    old_answers.write(&args.input);
}
