use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{AddAssign, Div, Mul};

// https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust
fn disemvowel(s: &str) -> String {
  // solution #1
  s.chars()
    .into_iter()
    .filter(|c| {
      *c != 'a'
        && *c != 'e'
        && *c != 'i'
        && *c != 'o'
        && *c != 'u'
        && *c != 'A'
        && *c != 'E'
        && *c != 'I'
        && *c != 'O'
        && *c != 'U'
    })
    .collect::<String>()

  // solution #2, best practice
  // s.chars()
  //   .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
  //   .collect()
}

#[test]
fn test_disemvowel() {
  assert_eq!(
    disemvowel("This website is for losers LOL!"),
    "Ths wbst s fr lsrs LL!"
  );
}

// https://www.codewars.com/kata/54bf1c2cd5b56cc47f0007a1/train/rust
fn count_duplicates(text: &str) -> u32 {
  // solution #1
  let counts = text
    .to_lowercase()
    .chars()
    .into_iter()
    .fold(HashMap::new(), |mut acc, c| {
      acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
      return acc;
    });
  counts
    .iter()
    .filter_map(|(&k, &v)| if v > 1 { Some(k) } else { None })
    .count() as u32

  // solution #2, use itertools, best practice
  // text.chars().duplicates_by(char::to_ascii_lowercase).count() as u32
}

#[test]
fn test_count_duplicates() {
  assert_eq!(count_duplicates("abcde"), 0);
  assert_eq!(count_duplicates("abcdea"), 1);
  assert_eq!(count_duplicates("indivisibility"), 1);
}

// https://www.codewars.com/kata/55a29405bc7d2efaff00007c/train/rust
// yet to submit as it still fails at higher test data, e.g. n = 464
fn going_zero_or_infinity(n: i32) -> f64 {
  fn factorial(n: i32) -> f64 {
    match n {
      1 => 1f64,
      _ => (n as f64).mul(factorial(n - 1)),
    }
  }
  fn sum_factorial(n: i32) -> f64 {
    let mut sum = 0f64;
    for i in 1..=n {
      sum.add_assign(factorial(i));
    }
    sum
  }
  // f64::max((1.0e-0_f64.div(factorial(n))).mul(sum_factorial(n)), 1.0e-40)
  1.0e-0_f64.div(factorial(n)).mul(sum_factorial(n))
}

use float_eq::float_eq;

#[test]
fn test_going_zero_or_infinity() {
  fn inner(n: i32, expected: f64) {
    let actual = going_zero_or_infinity(n);
    println!("n {} -- actual: {}", n, actual);
    let merr = 1.0e-6;
    let res = float_eq!(actual, expected, abs <= merr) || float_eq!(actual, expected, rmax <= merr);
    assert!(
      res,
      "For n = {n}\nExpected value must be near: {:e} but was:{:e}",
      expected, actual
    );
  }
  inner(5, 1.275);
  inner(6, 1.2125);
  inner(7, 1.173214);
  inner(8, 1.146651);
  inner(100, 1.0101020515541221);
  inner(464, 1.0021598373110507e0);
}

// https://www.codewars.com/kata/54da539698b8a2ad76000228/train/rust
fn is_valid_walk(walk: &[char]) -> bool {
  let str = walk.iter().collect::<String>();
  str.matches("n").count() == str.matches("s").count()
    && str.matches("w").count() == str.matches("e").count()
    && str.len() == 10
}

#[test]
fn test_is_valid_walk() {
  assert!(is_valid_walk(&[
    'n', 's', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
  ]));
  assert!(!is_valid_walk(&[
    'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e', 'w', 'e'
  ]));
  assert!(!is_valid_walk(&['w']));
  assert!(!is_valid_walk(&[
    'n', 'n', 'n', 's', 'n', 's', 'n', 's', 'n', 's'
  ]));
  assert!(!is_valid_walk(&[
    'e', 'e', 'e', 'e', 'w', 'w', 's', 's', 's', 's'
  ]))
}
