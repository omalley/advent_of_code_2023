use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum ModuleKind {
  Broadcast,
  FlipFlop,
  Conjunction,
  Inverter,
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

  fn print(&self, level: usize) {
    println!("{:level$}{} ({:?}):", "", self.name, self.kind);
  }

  fn sends_to(&self, target: usize) -> bool {
    self.outputs.iter().any(|e|
        e.as_ref().map_or(false, |edge| edge.target == target))
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
      // Conjunctions with a single input are just inverters
      if m.kind == ModuleKind::Conjunction && m.input_count == 1 {
        m.kind = ModuleKind::Inverter;
      }
    }
    Ok(Configuration{modules, broadcaster})
  }

  fn write_dot(&self, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(&mut file, "digraph {{")?;
    writeln!(&mut file, "  button [shape = invtriangle, rank = source]")?;
    writeln!(&mut file, "  button -> broadcaster")?;
    for module in &self.modules {
      match module.kind {
        ModuleKind::Broadcast => {
          writeln!(&mut file, "  {} [shape = box]", module.name)?;
        }
        ModuleKind::FlipFlop => {
          writeln!(&mut file, "  {} [shape = parallelogram]", module.name)?;
        }
        ModuleKind::Conjunction => {
          writeln!(&mut file, "  {} [shape = ellipse]", module.name)?;
        }
        ModuleKind::Inverter => {
          writeln!(&mut file, "  {} [shape = circle]", module.name)?;
        }
        ModuleKind::Output => {
          writeln!(&mut file, "  subgraph {{ rank = sink ; {} [shape = triangle] }}", module.name)?;
        }
      }
      for edge in module.outputs.iter() {
        match edge {
          None => writeln!(&mut file, "  {} -> unknown", module.name)?,
          Some(e) => writeln!(&mut file, "  {} -> {}", module.name, self.modules[e.target].name)?,
        }
      }
    }
    writeln!(&mut file, "}}")?;
    Ok(())
  }

  fn find_inputs(&self, target: usize) -> Vec<usize> {
    self.modules.iter().enumerate()
        .filter_map(|(i, m)| if m.sends_to(target) { Some(i) } else { None })
        .collect()
  }

  fn find_output_modules(&self) -> Vec<usize> {
    self.modules.iter().enumerate()
        .filter(|(_, m)| m.kind == ModuleKind::Output)
        .map(|(i, _)| i)
        .collect()
  }

  fn print_node_backtrace(&self, level: usize, stack: &mut Vec<usize>) {
    let current = *stack.last().unwrap();
    for child in self.find_inputs(current) {
      self.modules[child].print(2 * level);
      if !stack.contains(&child) {
        stack.push(child);
        self.print_node_backtrace(level + 1, stack);
        stack.pop();
      }
    }
  }

  fn print_backtrace(&self) {
    for output in self.find_output_modules() {
      self.modules[output].print(0);
      let mut stack = vec!{output};
      self.print_node_backtrace(1, &mut stack);
    }
  }
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum MessageKind {
  Low,
  High,
}

impl MessageKind {
  fn invert(&self) -> Self {
    match self {
      MessageKind::Low => MessageKind::High,
      MessageKind::High => MessageKind::Low,
    }
  }
}

#[derive(Clone,Debug)]
struct Message {
  kind: MessageKind,
  via: Edge,
}

enum State {
  Empty,
  FlipFlop(bool),
  Conjunction(Vec<MessageKind>),
}

impl State {
  fn new(module: &Module) -> Self {
    match module.kind {
      ModuleKind::Broadcast | ModuleKind::Inverter | ModuleKind::Output => State::Empty,
      ModuleKind::FlipFlop => State::FlipFlop(false),
      ModuleKind::Conjunction => State::Conjunction(vec![MessageKind::Low; module.input_count]),
    }
  }
}

struct FlowState<'a> {
  broadcaster: usize,
  modules: &'a [Module],
  states: Vec<State>,
  message_counts: [usize; 2],
  outputs: usize,
  pending: VecDeque<Message>,
}

impl<'a> FlowState<'a> {
  fn new(conf: &'a Configuration) -> Self {
    let broadcaster = conf.broadcaster;
    let modules = &conf.modules;
    let states = conf.modules.iter().map(|m| State::new(m)).collect();
    FlowState{broadcaster, modules, states, message_counts: [0; 2], outputs: 0,
      pending: VecDeque::new()}
  }

  fn send(&mut self, message: Message) {
    self.message_counts[message.kind as usize] += 1;
    self.pending.push_back(message);
  }

  fn part1_score(&self) -> usize {
    self.message_counts[0] * self.message_counts[1]
  }

  fn push_button(&mut self) {
    let via = Edge{target: self.broadcaster, input_num: 0};
    self.send(Message{kind: MessageKind::Low, via});
    self.stabilize();
  }

  fn stabilize(&mut self) {
    while let Some(message) = self.pending.pop_front() {
      let module = &self.modules[message.via.target];
      match module.kind {
        ModuleKind::Broadcast => {
          for edge in module.outputs.iter().flatten() {
            self.send(Message { kind: message.kind, via: edge.clone() })
          }
        }
        ModuleKind::FlipFlop => if message.kind == MessageKind::Low {
          if let State::FlipFlop(val) = &mut self.states[message.via.target] {
            *val = !*val;
            let kind = if *val { MessageKind::High } else { MessageKind::Low };
            for edge in module.outputs.iter().flatten() {
              self.send(Message { kind, via: edge.clone() });
            }
          }
        }
        ModuleKind::Conjunction => {
          if let State::Conjunction(prev) = &mut self.states[message.via.target] {
            prev[message.via.input_num] = message.kind;
            let kind = if prev.iter().all(|k| *k == MessageKind::High)
            { MessageKind::Low } else { MessageKind::High };
            for edge in module.outputs.iter().flatten() {
              self.send(Message { kind, via: edge.clone() });
            }
          }
        },
        ModuleKind::Inverter => {
          let kind = message.kind.invert();
          for edge in module.outputs.iter().flatten() {
            self.send(Message { kind, via: edge.clone() })
          }
        }
        ModuleKind::Output => {
          if message.kind == MessageKind::Low {
            self.outputs += 1;
          }
        }
      }
    }
  }
}

pub fn generator(input: &str) -> Configuration {
  Configuration::from_str(input).unwrap()
}

pub fn part1(input: &Configuration) -> usize {
  let mut state = FlowState::new(input);
  for _ in 0..1000 {
    state.push_button();
  }
  state.part1_score()
}

pub fn part2(input: &Configuration) -> u64 {
  //input.write_dot("day20.dot").unwrap();
  0
}

#[cfg(test)]
mod tests {
  use crate::day20::{generator, part1, part2};

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
&con -> rx";

  #[test]
  fn test_part1() {
    assert_eq!(32000000, part1(&generator(INPUT)));
    assert_eq!(11687500, part1(&generator(INPUT2)));
  }

  #[test]
  fn test_part2() {
    part2(&generator(INPUT2));
    //assert_eq!(0, part2(&generator(INPUT)));
  }
}
