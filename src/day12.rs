use std::collections::HashMap;
use std::iter;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Cell {
  Operational,
  Damaged,
  Unknown,
}

#[derive(Debug)]
struct Spring {
  cells: Vec<Cell>,
  groups: Vec<usize>,
}

#[derive(Default)]
struct Scatter {
  cache: HashMap<(usize, usize), usize>,
}

impl Spring {
  fn parse(text: &str) -> Spring {
    let (s1, s2) = text.split_once(' ').unwrap();
    let cells = s1.chars().map(|c| match c {
      '.' => Cell::Operational,
      '#' => Cell::Damaged,
      '?' => Cell::Unknown,
      _ => panic!("unknown symbol"),
    }).collect::<Vec<Cell>>();
    let groups = s2.split(',').map(|s| {
      s.parse::<usize>().unwrap()
    }).collect::<Vec<usize>>();
    Spring { cells, groups }
  }

  fn unfold(&self, count: usize) -> Spring {
    let concat = || iter::once(Cell::Unknown).chain(self.cells.iter().cloned());
    let cells = iter::repeat_with(concat)
      .take(count).flatten().skip(1).collect::<Vec<Cell>>();
    let groups = iter::repeat(self.groups.iter().cloned())
      .take(count).flatten().collect::<Vec<usize>>();
    Spring { cells, groups }
  }

  fn arrangements(&self) -> usize {
    let mut scatter = Scatter::default();
    descend(&self.cells[..], &self.groups[..], false, &mut scatter)
  }
}

impl Scatter {
  fn count(&mut self, n: usize, m: usize) -> usize {
    if n == 0 || m == 1 { return 1; }
    if let Some(res) = self.cache.get(&(n, m)) { return *res; }
    let res = (0..=n).map(|t| self.count(n - t, m - 1)).sum();
    self.cache.insert((n, m), res);
    res
  }
}

fn descend(cells: &[Cell], groups: &[usize],
           restrict: bool, scatter: &mut Scatter) -> usize {
  let some_is = |a: &[Cell], x: Cell| a.iter().any(|&c| c == x);
  if groups.is_empty() { return !some_is(cells, Cell::Damaged) as usize; }
  let next = groups[0];
  if next > cells.len() { return 0; }
  let size = cells.iter().take_while(|&&c| c == cells[0]).count();

  match cells[0] {
    Cell::Operational => descend(&cells[size..], groups, false, scatter),
    Cell::Damaged => {
      if restrict || size > next ||
         some_is(&cells[size..next], Cell::Operational) {0}
      else {descend(&cells[next..], &groups[1..], true, scatter)}
    },
    Cell::Unknown => {
      let mut count = descend(&cells[size..], groups, false, scatter);
      let mut available = size - restrict as usize;
      for fit in 1..=groups.len() {
        let last = groups[fit - 1];
        for tail in 1..=last {
          if tail > available { break; }
          let end = size + last - tail;
          if end > cells.len() { continue; }
          if !some_is(&cells[size..end], Cell::Operational) {
            let m = descend(&cells[end..], &groups[fit..], true, scatter);
            count += m * scatter.count(available - tail, fit);
          }
        }
        if last + 1 > available { break; }
        available -= last + 1;
        let m = descend(&cells[size..], &groups[fit..], false, scatter);
        for gaps in 0..=available {
          count += m * scatter.count(gaps, fit);
        }
      }
      count
    },
  }
}

pub fn run(content: &str) {
  let springs: Vec<Spring> = content.lines().map(Spring::parse).collect();
  let unfolded: Vec<Spring> = springs.iter().map(|x| x.unfold(5)).collect();
  let count = |x: &[Spring]| x.iter().enumerate().map(|(k, v)| {
    let res = v.arrangements();
    println!("{k} {res}");
    res
  }).sum::<usize>();
  println!("{} {}", count(&springs), count(&unfolded));
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

  #[test]
  fn small() {
    let test = TEST.lines().map(super::Spring::parse)
      .map(|x| x.arrangements()).collect::<Vec<usize>>();
    assert_eq!(test, [1, 4, 1, 1, 4, 10]);
  }

  #[test]
  fn large() {
    let test = TEST.lines().map(super::Spring::parse)
      .map(|x| x.unfold(5))
      .map(|x| x.arrangements()).collect::<Vec<usize>>();
    assert_eq!(test, [1, 16384, 1, 16, 2500, 506250]);
  }
}
