use std::collections::{HashMap, VecDeque};

const START: &str = "broadcaster";
const STOP: &str = "rx";

#[derive(Clone, Copy, Debug)]
enum Pulse {
  Low,
  High,
}

#[derive(Debug)]
enum Module {
  Broadcaster,
  FlipFlop(Pulse),
  Conjunction(HashMap<String, Pulse>),
}

#[derive(Debug)]
struct Node {
  name: String,
  module: Module,
  target: Vec<String>,
}

#[derive(Debug)]
struct Relay {
  nodes: HashMap<String, Node>,
  low: usize,
  high: usize,
  done: bool,
}

impl Pulse {
  fn inverse(&self) -> Pulse {
    match self {
      Pulse::Low => Pulse::High,
      Pulse::High => Pulse::Low,
    }
  }
}

impl Module {
  fn process(&mut self, signal: Pulse, from: &str) -> Option<Pulse> {
    match self {
      Module::Broadcaster => Some(signal),
      Module::FlipFlop(state) => {
        match signal {
          Pulse::High => None,
          Pulse::Low => {
            *state = state.inverse();
            Some(*state)
          },
        }
      },
      Module::Conjunction(cmap) => {
        *cmap.get_mut(from).unwrap() = signal;
        let all_high = cmap.values().all(|v| matches!(v, Pulse::High));
        Some(if all_high {Pulse::Low} else {Pulse::High})
      },
    }
  }
}

impl Node {
  fn parse(text: &str) -> Node {
    let (s1, s2) = text.split_once(" -> ").unwrap();
    let target = s2.split(", ").map(String::from).collect::<Vec<_>>();
    let (name, module) = match s1.chars().nth(0).unwrap() {
      '%' => (&s1[1..], Module::FlipFlop(Pulse::Low)),
      '&' => (&s1[1..], Module::Conjunction(HashMap::new())),
      _ => (s1, Module::Broadcaster),
    };
    Node { name: String::from(name), module, target }
  }
}

impl Relay {
  fn parse(text: &str) -> Relay {
    let mut nodes = text.lines().map(Node::parse)
      .map(|x| (x.name.clone(), x)).collect::<HashMap<_,_>>();
    let mut rmap = HashMap::<String, Vec<String>>::new();
    for node in nodes.values() {
      for t in &node.target {
        rmap.entry(t.clone()).or_default().push(node.name.clone());
      }
    }
    for (k, v) in rmap.into_iter() {
      if let Some(x) = nodes.get_mut(&k) {
        if let Module::Conjunction(cmap) = &mut x.module {
          cmap.extend(v.into_iter().map(|s| (s, Pulse::Low)));
        }
      }
    }
    assert!(nodes.contains_key(START));
    Relay { nodes, low: 0, high: 0, done: false }
  }

  fn press(&mut self) {
    let init = (String::new(), String::from(START), Pulse::Low);
    let mut queue = VecDeque::from([init]);
    while let Some((from, to, signal)) = queue.pop_front() {
      match signal {
        Pulse::Low => self.low += 1,
        Pulse::High => self.high += 1,
      };
      if let Some(node) = self.nodes.get_mut(&to) {
        if let Some(next) = node.module.process(signal, &from) {
          queue.extend(node.target.iter()
            .map(|v| (to.clone(), v.clone(), next)));
        }
      }
      if to == STOP && matches!(signal, Pulse::Low) {
        self.done = true;
      }
    }
  }

  fn repeat(&mut self, count: usize) -> usize {
    for _ in 0..count { self.press(); }
    self.low * self.high
  }

  fn wait_done(&mut self) -> usize {
    let mut count = 0_usize;
    while !self.done {
      self.press();
      count += 1;
    }
    count
  }
}

pub fn run(content: &str) {
  let mut relay = Relay::parse(content);
  let res1 = relay.repeat(1000);
  let res2 = relay.wait_done() + 1000;
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST_1: &str = "\
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
  const TEST_2: &str = "\
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";

  #[test]
  fn small() {
    let mut t1 = super::Relay::parse(TEST_1);
    let mut t2 = super::Relay::parse(TEST_2);
    assert_eq!(t1.repeat(1000), 32000000);
    assert_eq!(t2.repeat(1000), 11687500);
  }
}
