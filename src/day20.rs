use std::cmp::Reverse;
use std::collections::{HashMap, VecDeque};
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::prelude::*;
use itertools::Itertools;
use num_integer::Integer;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum ModuleKind {
  Broadcast,
  FlipFlop,
  Conjunction,
  Inverter,
  Output,
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
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

  #[allow(dead_code)]
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

  /// Find the nodes that lead up to a given target node
  fn slice(&self, target: usize) -> Vec<bool> {
    let mut result = vec![false; self.modules.len()];
    let mut pending = vec![target];
    while let Some(next) = pending.pop() {
      if !result[next] {
        result[next] = true;
        for prev in self.find_inputs(next) {
          pending.push(prev);
        }
      }
    }
    result
  }

  fn compute_partitions(&self) -> Option<Vec<Subgraph>> {
    if let [output] = &self.find_output_modules()[..] {
      if let [conditional] = &self.find_inputs(*output)[..] {
        if self.get_kind(*conditional) == ModuleKind::Conjunction {
          let mut result = Vec::new();
          for prev in self.find_inputs(*conditional) {
            result.push(Subgraph::init(self, self.broadcaster, prev,
                                       &self.slice(prev)));
          }
          return Some(result)
        }
      }
    }
    None
  }
}

trait Graph {
  fn start(&self) -> usize;
  fn next(&self, node: usize) -> &[Option<Edge>];
  fn is_exit(&self, node: usize) -> bool;
  fn num_nodes(&self) -> usize;
  fn get_name(&self, node: usize) -> &str;
  fn get_kind(&self, node: usize) -> ModuleKind;
  fn get_input_count(&self, node: usize) -> usize;
}

impl Graph for Configuration {
  fn start(&self) -> usize {
    self.broadcaster
  }

  fn next(&self, node: usize) -> &[Option<Edge>] {
    &self.modules[node].outputs
  }

  fn is_exit(&self, node: usize) -> bool {
    self.modules[node].kind == ModuleKind::Output
  }

  fn num_nodes(&self) -> usize {
    self.modules.len()
  }

  fn get_name(&self, node: usize) -> &str {
    &self.modules[node].name
  }

  fn get_kind(&self, node: usize) -> ModuleKind {
    self.modules[node].kind
  }

  fn get_input_count(&self, node: usize) -> usize {
    self.modules[node].input_count
  }
}

#[derive(Clone,Copy,Debug,Eq,Hash,PartialEq)]
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

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
struct Message {
  kind: MessageKind,
  via: Edge,
}

#[derive(Clone,Debug,Eq,Hash,PartialEq)]
enum State {
  Empty,
  FlipFlop(bool),
  Conjunction(Vec<MessageKind>),
}

impl State {
  fn new(kind: ModuleKind, input_count: usize) -> Self {
    match kind {
      ModuleKind::Broadcast | ModuleKind::Inverter | ModuleKind::Output => State::Empty,
      ModuleKind::FlipFlop => State::FlipFlop(false),
      ModuleKind::Conjunction => State::Conjunction(vec![MessageKind::Low; input_count]),
    }
  }
}

struct FlowState<'a> {
  graph: &'a dyn Graph,
  states: Vec<State>,
  message_counts: [usize; 2],
  outputs: [usize; 2],
  pending: VecDeque<Message>,
}

impl<'a> FlowState<'a> {
  fn new(graph: &'a dyn Graph) -> Self {
    let states = (0..graph.num_nodes())
        .map(|id| State::new(graph.get_kind(id), graph.get_input_count(id)))
        .collect();
    FlowState{graph, states, message_counts: [0; 2], outputs: [0; 2],
      pending: VecDeque::new()}
  }

  fn send(&mut self, kind: MessageKind, edge: &Option<Edge>) {
    self.message_counts[kind as usize] += 1;
    if let Some(e) = edge {
      self.pending.push_back(Message{kind, via: e.clone()});
    }
  }

  fn part1_score(&self) -> usize {
    self.message_counts.iter().product()
  }

  fn push_button(&mut self) {
    let via = Edge{target: self.graph.start(), input_num: 0};
    self.send(MessageKind::Low, &Some(via));
    self.stabilize();
  }

  fn stabilize(&mut self) {
    while let Some(message) = self.pending.pop_front() {
      let current = message.via.target;
      //println!("  {:?} to {current} {} ({:?})", message.kind,
      //         self.graph.get_name(current),
      //         self.graph.get_kind(current));
      match self.graph.get_kind(current) {
        ModuleKind::Broadcast => {
          for edge in self.graph.next(current) {
            self.send(message.kind, edge);
          }
        }
        ModuleKind::FlipFlop => if message.kind == MessageKind::Low {
          if let State::FlipFlop(val) = &mut self.states[current] {
            *val = !*val;
            let kind = if *val { MessageKind::High } else { MessageKind::Low };
            for edge in self.graph.next(current) {
              self.send(kind, edge);
            }
          }
        }
        ModuleKind::Conjunction => {
          if let State::Conjunction(prev) =
              &mut self.states[message.via.target] {
            prev[message.via.input_num] = message.kind;
            let kind =
                if prev.iter().all(|k| *k == MessageKind::High)
                    { MessageKind::Low } else { MessageKind::High };
            for edge in self.graph.next(current) {
              self.send(kind, edge);
            }
          }
        },
        ModuleKind::Inverter => {
          let kind = message.kind.invert();
          for edge in self.graph.next(current) {
            self.send(kind, edge);
          }
        }
        ModuleKind::Output => {
          self.outputs[message.kind as usize] += 1;
        }
      }
    }
  }
}

#[derive(Debug)]
struct Subgraph<'a> {
  graph: &'a Configuration,
  exit: usize,
  /// Indexed by module number in the graph, contains id number in subgraph.
  translation: Vec<usize>,
  /// Indexed by subgraph node id, leaving edges
  edges: Vec<Vec<Option<Edge>>>,
}

impl<'a> Subgraph<'a> {
  fn init(graph: &'a Configuration, start: usize, exit: usize, is_included: &[bool]) -> Self {
    let mut translation = Vec::new();
    let mut backwards = vec![None; graph.modules.len()];
    // build the translation for the subgraph
    let mut pending = vec![start];
    while let Some(current) = pending.pop() {
      if backwards[current].is_none() {
        backwards[current] = Some(translation.len());
        translation.push(current);
        for edge in graph.modules[current].outputs.iter().flatten() {
          if is_included[edge.target] {
            pending.push(edge.target);
          }
        }
      }
    }
    // Translate the edges to the new ids.
    let mut edges = translation.iter()
        .map(|old|
            graph.modules[*old].outputs.iter()
                .map(|e| e.as_ref()
                    .and_then(|e| backwards[e.target]
                        .map(|target| Edge{target, ..*e})))
                .collect())
        .collect::<Vec<Vec<Option<Edge>>>>();
    let mut output = exit;
    // Add the output node to the subgraph and connect the exit node
    // to it.
    if let Some(out) = graph.find_output_modules().first() {
      output = translation.len();
      translation.push(*out);
      edges[backwards[exit].unwrap()] = vec![Some(Edge{target: output, input_num: 0})];
      edges.push(vec![]);
    }
    Subgraph{graph, exit: output, translation, edges}
  }

  fn find_cycle(&self) -> CycleTracker {
    let mut states: HashMap<Vec<State>, usize> = HashMap::new();
    let mut time = 0;
    let mut state = FlowState::new(self);
    let mut outputs = Vec::new();
    let start;
    loop {
      state.outputs[MessageKind::High as usize] = 0;
      //println!("Pushing button {}",  time + 1);
      state.push_button();
      time += 1;
      if let Some(prev) = states.get(&state.states) {
        start = *prev;
        break;
      }
      states.insert(state.states.clone(), time);
      if state.outputs[MessageKind::High as usize] > 0 {
        outputs.push(time);
      }
    }
    CycleTracker{outputs, start, length: time - start}
  }
}

impl<'a> Graph for Subgraph<'a> {
  fn start(&self) -> usize {
    0
  }

  fn next(&self, node: usize) -> &[Option<Edge>] {
    &self.edges[node]
  }

  fn is_exit(&self, node: usize) -> bool {
    node == self.exit
  }

  fn num_nodes(&self) -> usize {
    self.translation.len()
  }

  fn get_name(&self, node: usize) -> &str {
    &self.graph.modules[self.translation[node]].name
  }

  fn get_kind(&self, node: usize) -> ModuleKind {
    self.graph.modules[self.translation[node]].kind
  }

  fn get_input_count(&self, node: usize) -> usize {
    self.graph.modules[self.translation[node]].input_count
  }
}

impl<'a> Display for Subgraph<'a> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    for (id, trans) in self.translation.iter().enumerate() {
      let module = &self.graph.modules[*trans];
      writeln!(f, "  {id} {} {:?}", module.name, module.kind)?;
      for edge in &self.edges[id] {
        writeln!(f, "    {:?}", edge)?;
      }
    }
    Ok(())
  }
}

#[derive(Clone,Copy,Debug)]
struct Recurrence {
  cycle: usize,
  remainder: usize,
}

impl Recurrence {
  fn solve(recurrences: &[Recurrence]) -> usize {
    // Build an index that sorts recurrences by descending cycle size.
    if recurrences.is_empty() {
      return 0;
    }
    let mut index = (0..recurrences.len()).collect::<Vec<usize>>();
    index.sort_unstable_by_key(|x| Reverse(recurrences[*x].cycle));
    let mut result = recurrences[index[0]].remainder;
    let mut skip_size = recurrences[index[0]].cycle;
    if result == 0 {
      result += skip_size;
    }
    for i in &index[1..] {
      let recurrence = &recurrences[*i];
      if recurrence.remainder == 0 {
        result = result.lcm(&recurrence.cycle);
      } else {
        while result % recurrence.cycle != recurrence.remainder {
          result += skip_size;
        }
      }
      skip_size = skip_size.lcm(&recurrence.cycle);
    }
    result
  }
}

#[derive(Clone,Debug)]
struct CycleTracker {
  // The times when we got lows
  outputs: Vec<usize>,
  // The start of the first cycle
  start: usize,
  // the length of the cycle
  length: usize,
}

impl CycleTracker {
  fn solve(cycles: &[CycleTracker]) -> Option<usize> {
    if cycles.iter().any(|c| c.start != 1) {
      return None;
    }
    Some(cycles.iter().map(|cycle| cycle.outputs.iter()
        .map(|t| Recurrence{cycle: cycle.length, remainder: *t % cycle.length}))
        .multi_cartesian_product()
        .map(|cp| Recurrence::solve(&cp))
        .min().unwrap())
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

fn brute_force_part2(input: &Configuration) -> usize {
  let mut state = FlowState::new(input);
  let mut time = 0;
  while state.outputs[MessageKind::Low as usize] == 0 {
    state.push_button();
    time += 1;
  }
  time
}

pub fn part2(input: &Configuration) -> usize {
  if let Some(parts) = input.compute_partitions() {
    if let Some(answer) = CycleTracker::solve(&parts.iter()
        .map(|p| p.find_cycle()).collect::<Vec<CycleTracker>>()) {
      return answer;
    }
  }
  brute_force_part2(input)
}

#[cfg(test)]
mod tests {
  use crate::day20::{generator, part1, part2, Recurrence};

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
    assert_eq!(1, part2(&generator(INPUT2)));
  }

  #[test]
  fn test_recurrence() {
    let data = vec![Recurrence{cycle: 4051, remainder: 0},
                    Recurrence{cycle: 4021, remainder: 0},
                    Recurrence{cycle: 4057, remainder: 0},
                    Recurrence{cycle: 3833, remainder: 0}];
    assert_eq!(253302889093151, Recurrence::solve(&data));
  }
}
