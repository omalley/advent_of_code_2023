use std::collections::{HashMap, VecDeque};

#[derive(Clone,Copy,Debug)]
pub enum ModuleKind {
  Broadcast,
  FlipFlop,
  Conjunction,
  Output,
}

#[derive(Clone,Debug)]
pub struct Edge {
  target: usize,
  input_num: usize,
}

#[derive(Clone,Debug)]
pub struct Module {
  #[allow(dead_code)]
  name: String,
  kind: ModuleKind,
  outputs: Vec<Option<Edge>>,
  input_count: usize,
}

impl Module {
  fn from_str(s: &str, names: &HashMap<String,usize>,
              input_counts: &mut [usize]) -> Result<Self,String> {
    let (full_name, target) = s.split_once(" -> ")
        .ok_or(format!("Can't parse module - {s}"))?;
    let name;
    let kind;
    if let Some(tail) = full_name.strip_prefix('%') {
      name = tail;
      kind = ModuleKind::FlipFlop;
    } else if let Some(tail) = full_name.strip_prefix('&') {
      name = tail;
      kind = ModuleKind::Conjunction;
    } else if full_name == "broadcaster" {
      name = full_name;
      kind = ModuleKind::Broadcast;
    } else {
      return Err(format!("Can't determine kind in {s}"));
    }
    let outputs = target.split(", ")
        .map(|s| names.get(s).map(|&u| {
          let in_cnt = input_counts[u];
          input_counts[u] += 1;
          Edge{target: u, input_num: in_cnt}})).collect();
    Ok(Module{name: name.to_string(), kind, outputs, input_count: 0})
  }
}

#[derive(Clone,Debug)]
pub struct Configuration {
  modules: Vec<Module>,
  broadcaster: usize,
}

impl Configuration {
  const FINAL_STATE_NAME: &'static str = "rx";

  fn from_str(s: &str) -> Result<Self,String> {
    let mut names = HashMap::new();
    for (i, line) in s.lines().enumerate() {
      let (name, _) = line.split_once(" -> ")
          .ok_or(format!("Can't find name in {line}"))?;
      let mut name = name.to_string();
      if name.starts_with('%') || name.starts_with('&') {
        name.remove(0);
      }
      names.insert(name, i);
    }
    let adding_final_state = !names.contains_key(Self::FINAL_STATE_NAME);
    if adding_final_state {
      names.insert(Self::FINAL_STATE_NAME.to_string(), names.len());
    }
    let broadcaster = *names.get("broadcaster").ok_or("Can't find broadcaster")?;
    let mut input_counts = vec![0; names.len()];
    let mut modules = s.lines()
        .map(|l| Module::from_str(l, &names, &mut input_counts))
        .collect::<Result<Vec<Module>,String>>()?;
    if adding_final_state {
      modules.push(Module{name: Self::FINAL_STATE_NAME.to_string(),
        kind: ModuleKind::Output, outputs: Vec::new(), input_count: 0})
    }
    for (i, m) in modules.iter_mut().enumerate() {
      m.input_count = input_counts[i];
    }
    Ok(Configuration{modules, broadcaster})
  }

  fn push_button(&self, state: &mut [State]) -> ([usize; 2], [usize; 2]) {
    let mut count = [0; 2];
    let mut output_count = [0; 2];
    let mut pending: VecDeque<Message> = VecDeque::new();
    count[MessageKind::Low as usize] += 1;
    pending.push_back(Message{kind: MessageKind::Low,
      via: Edge{target: self.broadcaster, input_num: 0}});
    while let Some(current) = pending.pop_front() {
      let module = &self.modules[current.via.target];
      match module.kind {
        ModuleKind::Broadcast => {
          count[current.kind as usize] += module.outputs.len();
          for edge in module.outputs.iter().flatten() {
            pending.push_back(Message{kind: current.kind, via: edge.clone()})
          }
        }
        ModuleKind::FlipFlop => if current.kind == MessageKind::Low {
          if let State::FlipFlop(val) = &mut state[current.via.target] {
            *val = !*val;
            let kind = if *val { MessageKind::High } else { MessageKind::Low };
            count[kind as usize] += module.outputs.len();
            for edge in module.outputs.iter().flatten() {
              pending.push_back(Message{kind, via: edge.clone()});
            }
          }
        }
        ModuleKind::Conjunction => {
          if let State::Conjunction(prev) = &mut state[current.via.target] {
            prev[current.via.input_num] = current.kind;
            let kind = if prev.iter().all(|k| *k == MessageKind::High)
              { MessageKind::Low } else { MessageKind::High };
            count[kind as usize] += module.outputs.len();
            for edge in module.outputs.iter().flatten() {
              pending.push_back(Message{kind, via: edge.clone()});
            }
          }
        },
        ModuleKind::Output => {
          output_count[current.kind as usize] += 1;
        }
      }
    }
    (count, output_count)
  }
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum MessageKind {
  Low,
  High,
}

#[derive(Clone,Debug)]
struct Message {
  kind: MessageKind,
  via: Edge,
}

enum State {
  Broadcast,
  FlipFlop(bool),
  Conjunction(Vec<MessageKind>),
  Output,
}

impl State {
  fn new(conf: &Configuration) -> Vec<Self> {
    conf.modules.iter().map(|m| match m.kind {
      ModuleKind::Broadcast => State::Broadcast,
      ModuleKind::FlipFlop => State::FlipFlop(false),
      ModuleKind::Conjunction => State::Conjunction(vec![MessageKind::Low; m.input_count]),
      ModuleKind::Output => State::Output,
    }).collect()
  }
}

pub fn generator(input: &str) -> Configuration {
  Configuration::from_str(input).unwrap()
}

pub fn part1(input: &Configuration) -> usize {
  let mut state = State::new(input);
  let mut count = [0; 2];
  for _ in 0..1000 {
    for (i, c) in input.push_button(&mut state).0.iter().enumerate() {
      count[i] += *c;
    }
  }
  count[0] * count[1]
}

pub fn part2(input: &Configuration) -> u64 {
  for m in &input.modules {
    println!("{:?}", m);
  }
  let mut state = State::new(input);
  for count in 0..u64::MAX {
    if count % 100_000 == 0 {
      println!("count = {count}");
    }
    let (_, output) = input.push_button(&mut state);
    if output == [1, 0] {
      return count
    }
  }
  panic!("Not found")
}

#[cfg(test)]
mod tests {
  use crate::day20::{generator, part1};

  const INPUT: &str =
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

  const INPUT2: &str =
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

  #[test]
  fn test_part1() {
    assert_eq!(32000000, part1(&generator(INPUT)));
    assert_eq!(11687500, part1(&generator(INPUT2)));
  }

  #[test]
  fn test_part2() {
    //assert_eq!(0, part2(&generator(INPUT)));
  }
}
