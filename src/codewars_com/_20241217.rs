use itertools::Itertools;
use std::collections::HashMap;

// https://www.codewars.com/kata/5aa736a455f906981800360d/train/rust
fn feast(beast: &str, dish: &str) -> bool {
  let a = beast.to_lowercase().chars().collect::<Vec<_>>();
  let b = dish.to_lowercase().chars().collect::<Vec<_>>();
  a[0] == b[0] && a[a.len() - 1] == b[b.len() - 1]
}

#[test]
fn test_feast() {
  assert_eq!(feast("great blue heron", "garlic naan"), true);
  assert_eq!(feast("chickadee", "chocolate cake"), true);
  assert_eq!(feast("brown bear", "bear claw"), false);
}

// https://www.codewars.com/kata/52efefcbcdf57161d4000091/train/rust
fn count(input: &str) -> HashMap<char, i32> {
  input.chars().fold(HashMap::new(), |mut map, ch| {
    *map.entry(ch).or_insert(0) += 1;
    map
  })

  // solution #2, use itertools
  // input.chars()
  //   .counts()
  //   .into_iter()
  //   .map(|(k, v)| (k, v as i32))
  //   .collect()
}

#[test]
fn test_count() {
  const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";
  let test_empty_string = || {
    let test_input = "";
    let expected: HashMap<char, i32> = HashMap::new();
    assert_eq!(
      count(test_input),
      expected,
      "{ERR_MSG} with input: \"{test_input}\""
    );
  };
  let test_string_with_two_equal_letters = || {
    let test_input = "aa";
    let mut expected: HashMap<char, i32> = HashMap::new();
    expected.insert('a', 2);
    assert_eq!(
      count(test_input),
      expected,
      "{ERR_MSG} with input: \"{test_input}\""
    );
  };
  let test_string_with_different_letters = || {
    let test_input = "aabb";
    let mut expected: HashMap<char, i32> = HashMap::new();
    expected.insert('a', 2);
    expected.insert('b', 2);
    assert_eq!(
      count(test_input),
      expected,
      "{ERR_MSG} with input: \"{test_input}\""
    );
  };
  test_empty_string();
  test_string_with_two_equal_letters();
  test_string_with_different_letters();
}

// https://www.codewars.com/kata/5552101f47fc5178b1000050/train/rust
fn dig_pow(n: i64, p: i32) -> i64 {
  // solution #1
  let s = n
    .to_string()
    .chars()
    .map(|c| c.to_string().parse::<i64>().unwrap())
    .collect::<Vec<_>>();
  let mut p2 = p;
  let mut res = 0;
  s.iter().for_each(|&a| {
    res += a.pow(p2 as u32);
    p2 += 1;
  });

  // solution #2
  // let r: i64 = n.to_string().chars()
  //   .map(|c| (c as i64) - 48)
  //   .enumerate()
  //   .map(|(i, d)| i64::pow(d, p as u32 + i as u32))
  //   .sum();

  match res % n {
    0 => res / n,
    _ => -1,
  }
}

#[test]
fn basic_tests() {
  let inner = |n: i64, p: i32, exp: i64| -> () {
    println!(" n: {:?};", n);
    println!("p: {:?};", p);
    let ans = dig_pow(n, p);
    println!(" actual:\n{:?};", ans);
    println!("expect:\n{:?};", exp);
    println!(" {};", ans == exp);
    assert_eq!(ans, exp);
    println!("{};", "-");
  };
  inner(89, 1, 1);
  inner(92, 1, -1);
  inner(46288, 3, 51);
}
