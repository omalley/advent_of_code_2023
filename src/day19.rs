use std::collections::HashMap;

type Rating = i32;
type RuleId = u32;

#[derive(Clone,Copy,Debug)]
pub enum AttributeId {
  X,
  M,
  A,
  S,
}

impl AttributeId {
  fn from_str(s: &str) -> Result<Self,String> {
    match s {
      "x" => Ok(AttributeId::X),
      "m" => Ok(AttributeId::M),
      "a" => Ok(AttributeId::A),
      "s" => Ok(AttributeId::S),
      _ => Err(format!("Unknown attribute {s}")),
    }
  }
}

const NUM_ATTRIBUTES: usize = 4;

#[derive(Clone,Debug,Default)]
pub struct Part {
  ratings: [Rating; NUM_ATTRIBUTES],
}

impl Part {
  fn from_str(s: &str) -> Result<Self,String> {
    let mut result = Self::default();
    for phrase in s.strip_prefix('{').ok_or("No open bracket")?
        .strip_suffix('}').ok_or("No closing bracket")?
        .split(',') {
      let (attr, value) = phrase.split_once('=')
          .ok_or(format!("Can't parse {phrase}"))?;
      result.ratings[AttributeId::from_str(attr)? as usize] =
          value.parse::<Rating>().map_err(|_| format!("can't parse integer {value}"))?;
    }
    Ok(result)
  }

  fn value(&self) -> i64 {
    self.ratings.iter().map(|v| *v as i64).sum()
  }
}

#[derive(Clone,Copy,Debug)]
pub enum Target {
  GoTo(RuleId),
  Accept,
  Reject,
}

impl Target {
  fn from_str(s: &str, names: &HashMap<String,RuleId>) -> Result<Self,String> {
    match s {
      "A" => Ok(Target::Accept),
      "R" => Ok(Target::Reject),
      rule => Ok(Target::GoTo(*names.get(rule).ok_or(format!("Can't find rule {rule}"))?))
    }
  }
}

fn parse_rating(s: &str) -> Result<Rating,String> {
  s.parse().map_err(|_| format!("Can't parse integer - {s}"))
}

#[derive(Clone,Debug)]
pub enum Operator {
  Less(Rating),
  Greater(Rating),
}

impl Operator {
  fn from_str(s: &str) -> Result<Self,String> {
    if let Some(tail) = s.strip_prefix('<') {
      Ok(Operator::Less(parse_rating(tail)?))
    } else if let Some(tail) = s.strip_prefix('>') {
      Ok(Operator::Greater(parse_rating(tail)?))
    } else {
      Err(format!("Can't find operator in {s}"))
    }
  }

  fn evaluate(&self, val: Rating) -> bool {
    match self {
      Operator::Less(target) => val < *target,
      Operator::Greater(target) => val > *target,
    }
  }
}

#[derive(Clone,Debug)]
pub struct RuleCondition {
  attribute: AttributeId,
  operator: Operator,
  target: Target,
}

impl RuleCondition {
  fn from_str(s: &str, names: &HashMap<String,RuleId>) -> Result<Self,String> {
    let (cond, target_name) = s.split_once(':')
        .ok_or("Can't find target in {s}")?;
    let attribute = AttributeId::from_str(&cond[..1])?;
    let operator = Operator::from_str(&cond[1..])?;
    let target = Target::from_str(target_name, names)?;
    Ok(RuleCondition{attribute,operator,target})
  }

  fn evaluate(&self, part: &Part) -> Option<Target> {
    if self.operator.evaluate(part.ratings[self.attribute as usize]) {
      Some(self.target)
    } else {
      None
    }
  }
}

#[derive(Clone,Debug)]
pub struct Rule {
  name: String,
  conditions: Vec<RuleCondition>,
  last: Target,
}

impl Rule {
  fn from_str(s: &str, names: &HashMap<String, RuleId>) -> Result<Self,String> {
    let (name, definition) = s.split_once('{')
        .ok_or("Can't find definition in {s}")?;
    let definition = definition.strip_suffix('}')
        .ok_or(format!("Can't find closing brace in {s}"))?;
    let conds = definition.split(',').collect::<Vec<&str>>();
    let conditions = conds[..conds.len()-1].iter()
        .map(|s| RuleCondition::from_str(s, names))
        .collect::<Result<Vec<RuleCondition>,String>>()?;
    let last = Target::from_str(conds.last()
        .ok_or(format!("No definitions in {definition}"))?, names)?;
    Ok(Rule{name: name.to_string(), conditions, last})
  }

  fn evalute(&self, part: &Part) -> Target {
    for c in &self.conditions {
      if let Some(target) = c.evaluate(part) {
        return target;
      }
    }
    self.last
  }
}

#[derive(Clone,Debug)]
pub struct Input {
  rule_set: Vec<Rule>,
  start_rule: RuleId,
  parts: Vec<Part>,
}

impl Input {
  pub fn from_str(s: &str) -> Result<Self,String> {
    let (rule_str, part_str) = s.split_once("\n\n")
        .ok_or("can't find part lists")?;
    let names = rule_str.lines().enumerate()
        .map(|(i, line)|
          line.split_once('{')
              .ok_or(format!("can't find attribute name in {line}"))
              .map(|(name, _)| (name.to_string(), i as RuleId)))
        .collect::<Result<HashMap<String, RuleId>,String>>()?;
    let rule_set = rule_str.lines().map(|s| Rule::from_str(s, &names))
        .collect::<Result<Vec<Rule>,String>>()?;
    let parts = part_str.lines().map(|l| Part::from_str(l))
        .collect::<Result<Vec<Part>,String>>()?;
    Ok(Input{rule_set, parts,
      start_rule: *names.get("in").ok_or("Can't find in rule".to_string())?})
  }

  pub fn accept(&self, part: &Part) -> bool {
    let mut rule = self.start_rule;
    loop {
      match self.rule_set[rule as usize].evalute(part) {
        Target::GoTo(next) => rule = next,
        Target::Accept => return true,
        Target::Reject => return false,
      }
    }
  }
}

pub fn generator(input: &str) -> Input {
  Input::from_str(input).unwrap()
}

pub fn part1(input: &Input) -> i64 {
  input.parts.iter().filter(|&p| input.accept(p)).map(|p| p.value()).sum()
}

pub fn part2(input: &Input) -> i64 {
  0
}

#[cfg(test)]
mod tests {
  use crate::day19::{generator, part1, part2};

  const INPUT: &str =
"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

  #[test]
  fn test_part1() {
    assert_eq!(19114, part1(&generator(INPUT)));
  }

  #[test]
  fn test_part2() {
    assert_eq!(952408144115, part2(&generator(INPUT)));
  }
}
