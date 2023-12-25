use std::collections::HashMap;

type NodeId = u16;

#[derive(Clone,Debug,Eq,Ord,PartialEq,PartialOrd)]
pub struct Edge {
  nodes: [NodeId; 2],
}

#[derive(Clone,Debug)]
pub struct Graph {
  nodes: Vec<String>,
  edges: Vec<Edge>,
}

impl Graph {
  fn parse_line(line: &str) -> Result<Vec<String>,String> {
    let (left, right) = line.split_once(": ").ok_or("Can't parse {line}")?;
    let mut result = vec![left.to_string()];
    for next in right.split_whitespace() {
      result.push(next.to_string());
    }
    Ok(result)
  }

  fn from_str(s: &str) -> Result<Self,String> {
    let mut names: HashMap<String, NodeId> = HashMap::new();
    let mut nodes = Vec::new();
    for line in s.lines() {
      for word in Self::parse_line(line)? {
        names.entry(word.clone()).or_insert_with(|| {
          nodes.push(word.clone());
          nodes.len() as NodeId - 1
        });
      }
    }
    let mut edges = Vec::new();
    for line in s.lines() {
      let parts = Self::parse_line(line)?;
      let current = names.get(&parts[0]).unwrap();
      for other_name in &parts[1..] {
        let other = names.get(other_name).unwrap();
        let mut nodes = [*current, *other];
        nodes.sort_unstable();
        edges.push(Edge{nodes});
      }
    }
    edges.sort_unstable();
    Ok(Graph{nodes,edges})
  }

  fn is_partitioned(&self, removed: &[usize]) -> Option<(usize,usize)> {
    let mut reachable = vec![false; self.nodes.len()];
    reachable[0] = true;
    let mut count = 1;
    let mut changes = true;
    while changes {
      changes = false;
      for (i, edge) in self.edges.iter().enumerate() {
        if !removed.contains(&i) && reachable[edge.nodes[0] as usize] != reachable[edge.nodes[1] as usize] {
          count += 1;
          changes = true;
          if reachable[edge.nodes[0] as usize] {
            reachable[edge.nodes[1] as usize] = true;
          } else {
            reachable[edge.nodes[0] as usize] = true;
          }
        }
      }
    }
    if count != reachable.len() {
      Some((count, reachable.len() - count))
    } else {
      None
    }
  }
}

pub fn generator(input: &str) -> Graph {
  Graph::from_str(input).unwrap()
}

pub fn part1(graph: &Graph) -> usize {
  for e1 in 1..graph.edges.len() {
    for e2 in e1+1..graph.edges.len() {
      for e3 in e2+1..graph.edges.len() {
        if let Some((part1, part2)) = graph.is_partitioned(&[e1, e2, e3]) {
          return part1 * part2
        }
      }
    }
  }
  0
}

pub fn part2(_input: &Graph) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use crate::day25::{generator, part1};

  const INPUT: &str =
"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

  #[test]
  fn test_part1() {
    assert_eq!(54, part1(&generator(INPUT)));
  }
}
