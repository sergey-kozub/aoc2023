use std::collections::HashSet;
use std::fmt;

type Point = (i32, i32);

struct Field {
  walls: HashSet<Point>,
  steps: HashSet<Point>,
  size: Point,
}

impl Field {
  fn parse(text: &str) -> Field {
    let width = text.lines().next().unwrap().len() as i32;
    let height = text.lines().count() as i32;
    let mut start: Option<Point> = None;
    let walls = text.lines().enumerate().flat_map(|(y, s)| {
      let mut a: Vec<Point> = vec![];
      for (x, c) in s.chars().enumerate() { match c {
        '#' => a.push((x as i32, y as i32)),
        '.' => (),
        'S' => start = Some((x as i32, y as i32)),
        _ => panic!("unknown symbol"),
      }}
      a.into_iter()
    }).collect::<HashSet<_>>();
    let steps = HashSet::from([start.unwrap()]);
    Field { walls, steps, size: (width, height) }
  }

  fn next_step(self) -> Field {
    let steps = self.steps.iter().flat_map(|&(x, y)| {
      [
        if x > 0 {Some((x - 1, y))} else {None},
        if x < self.size.0 - 1 {Some((x + 1, y))} else {None},
        if y > 0 {Some((x, y - 1))} else {None},
        if y < self.size.1 - 1 {Some((x, y + 1))} else {None},
      ].into_iter().filter_map(|t| {
        t.filter(|p| !self.walls.contains(p) && !self.steps.contains(p))
      })
    }).collect::<HashSet<_>>();
    Field { walls: self.walls, steps, size: self.size }
  }

  fn forward(self, count: usize) -> Field {
    (0..count).fold(self, |acc, _| acc.next_step())
  }
}

impl fmt::Debug for Field {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", (0..self.size.1).fold(String::new(), |s, y| {
      s + &String::from_utf8((0..self.size.0).map(|x| (
        if self.steps.contains(&(x, y)) {'O'} else
        if self.walls.contains(&(x, y)) {'#'} else {'.'}
      ) as u8).collect::<Vec<_>>()).unwrap() + "\n"
    }))
  }
}

pub fn run(content: &str) {
  let field = Field::parse(content);
  let res1 = field.forward(64).steps.len();
  println!("{}", res1);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

  #[test]
  pub fn small() {
    let test = super::Field::parse(TEST).forward(6);
    assert_eq!(test.steps.len(), 16);
  }
}
