use std::ops::RangeInclusive;

type Point = (i64, i64, i64);
type Fraction = (i128, i128);

#[derive(Debug)]
struct Particle {
  position: Point,
  velocity: Point,
}

#[derive(Debug)]
struct Line2D {
  a: i64,
  b: i64,
  c: i64,
}

impl Particle {
  fn parse(text: &str) -> Particle {
    let sub = |s: &str| -> Point {
      let a = s.split(", ")
        .map(|x| x.trim_matches(' ').parse::<i64>().unwrap())
        .collect::<Vec<_>>();
      (a[0], a[1], a[2])
    };
    let (s1, s2) = text.split_once(" @ ").unwrap();
    Particle { position: sub(s1), velocity: sub(s2) }
  }

  fn count_2d(&self, other: &[Particle], range: &RangeInclusive<i64>) -> usize {
    let line = Line2D::new(self);
    other.iter().filter(|part| {
      let (x, y) = line.intersection(&Line2D::new(part));
      within(&x, range) && within(&y, range) &&
      !is_past(&x, self.position.0, self.velocity.0) &&
      !is_past(&x, part.position.0, part.velocity.0)
    }).count()
  }
}

impl Line2D {
  fn new(part: &Particle) -> Line2D {
    let (x1, y1) = (part.position.0, part.position.1);
    let (x2, y2) = (x1 + part.velocity.0, y1 + part.velocity.1);
    let a = y2 - y1;
    let b = x1 - x2;
    let c = -b * y1 - a * x1;
    Line2D { a, b, c }
  }

  fn intersection(&self, other: &Line2D) -> (Fraction, Fraction) {
    let (a1, b1, c1) = (self.a as i128, self.b as i128, self.c as i128);
    let (a2, b2, c2) = (other.a as i128, other.b as i128, other.c as i128);
    let d = a1 * b2 - a2 * b1;
    let norm = |v: i128| { if d >= 0 {(v, d)} else {(-v, -d)} };
    (norm(b1 * c2 - b2 * c1), norm(a2 * c1 - a1 * c2))
  }
}

fn within(frac: &Fraction, range: &RangeInclusive<i64>) -> bool {
  frac.0 >= *range.start() as i128 * frac.1 &&
  frac.0 <= *range.end() as i128 * frac.1
}

fn is_past(frac: &Fraction, pos: i64, inc: i64) -> bool {
  let (a, b) = (frac.0, pos as i128 * frac.1);
  if a != b {(a > b) == (inc < 0)} else {false}
}

pub fn run(content: &str) {
  let data = content.lines().map(Particle::parse).collect::<Vec<_>>();
  let range = 200_000_000_000_000..=400_000_000_000_000_i64;
  let res1 = (0..data.len()).map(|i| data[i].count_2d(&data[i+1..], &range))
    .sum::<usize>();
  println!("{}", res1);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";

  #[test]
  fn small() {
    let a = TEST.lines().map(super::Particle::parse).collect::<Vec<_>>();
    let it = (0..a.len()).map(|i| a[i].count_2d(&a[i+1..], &(7..=27)));
    assert_eq!(it.collect::<Vec<_>>(), [2, 0, 0, 0, 0]);
  }
}
