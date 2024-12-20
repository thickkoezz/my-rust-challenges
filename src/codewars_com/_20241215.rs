// https://www.codewars.com/kata/578553c3a1b8d5c40300037c/train/rust
fn binary_slice_to_number(slice: &[u32]) -> u32 {
  // solution #1
  let str = slice
    .iter()
    .map(|x| format!("{x}"))
    .collect::<Vec<String>>()
    .concat();
  u32::from_str_radix(str.as_str(), 2).unwrap()

  // solution #2, best practice
  // slice.iter().fold(0, |acc, bit| (acc << 1) | bit)
}

#[test]
fn test_binary_slice_to_number() {
  assert_eq!(binary_slice_to_number(&vec![0, 0, 0, 1]), 1);
  assert_eq!(binary_slice_to_number(&vec![0, 0, 1, 0]), 2);
  assert_eq!(binary_slice_to_number(&vec![1, 1, 1, 1]), 15);
  assert_eq!(binary_slice_to_number(&vec![0, 1, 1, 0]), 6);
}

// https://www.codewars.com/kata/551b4501ac0447318f0009cd/rust
fn odd_or_even(numbers: Vec<i32>) -> String {
  match numbers.iter().sum::<i32>() % 2 {
    0 => "even".to_string(),
    _ => "odd".to_string(),
  }
}

#[test]
fn test_odd_or_even() {
  assert_eq!(odd_or_even(vec![]), "even");
  assert_eq!(odd_or_even(vec![0]), "even");
  assert_eq!(odd_or_even(vec![1]), "odd");
  assert_eq!(odd_or_even(vec![0, 1, 5]), "even");
  assert_eq!(odd_or_even(vec![0, 1, 4]), "odd");
  assert_eq!(odd_or_even(vec![0, -1, -5]), "even");
  assert_eq!(odd_or_even(vec![0, -1, 2]), "odd");
}

// https://www.codewars.com/kata/55908aad6620c066bc00002a/train/rust
fn xo(string: &'static str) -> bool {
  // solution #1
  let x = string.to_lowercase().chars().filter(|&c| c == 'x').count();
  let o = string.to_lowercase().chars().filter(|&c| c == 'o').count();
  x == o

  // solution #2, best practice
  // let s = string.to_lowercase();
  // s.matches("x").count() == s.matches("o").count()
}

#[test]
fn test_xo() {
  assert_eq!(xo("xo"), true);
  assert_eq!(xo("Xo"), true);
  assert_eq!(xo("xxOo"), true);
  assert_eq!(xo("xxxm"), false);
  assert_eq!(xo("Oo"), false);
  assert_eq!(xo("ooom"), false);
}

use itertools::Itertools;

// https://www.codewars.com/kata/578aa45ee9fd15ff4600090d/train/rust
fn sort_array(arr: &[i32]) -> Vec<i32> {
  // solution #1
  let mut odds = arr
    .iter()
    .map(|&x| x)
    .filter(|&x| x % 2 > 0)
    .collect::<Vec<i32>>();
  odds.sort();
  odds.reverse();
  arr
    .into_iter()
    .map(|&x| if x % 2 == 0 { x } else { odds.pop().unwrap() })
    .collect::<Vec<i32>>()

  // solution #2: using itertools, best practice
  // let mut odds = arr.iter().filter(|&x| x % 2 != 0).sorted();
  // arr.iter().map(|x| if x % 2 != 0 { odds.next().unwrap() } else { x }).cloned().collect()
}

#[test]
fn test_sort_array() {
  assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
  assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
  assert_eq!(sort_array(&[]), []);
}

// https://www.codewars.com/kata/5208f99aee097e6552000148/train/rust
fn break_camel_case(s: &str) -> String {
  // solution #1
  let mut iter = s.chars().into_iter().peekable();
  let mut next = iter.next();
  let mut str = String::new();
  while next.is_some() {
    str += &next.unwrap().to_string();
    if let Some(mut p) = iter.peek_mut() {
      if p.is_uppercase() {
        str += " ";
      }
    }
    next = iter.next();
  }
  str

  // solution #2
  // let mut res = String::new();
  // for c in s.chars() {
  //   if c.is_uppercase() {
  //     res.push(' ');
  //   }
  //   res.push(c);
  // }
  // res

  // solution #3, best practice
  // s.chars().map(|c| if c.is_uppercase() { format!(" {}", c) } else { format!("{}", c) }).collect()
}

#[test]
fn test_break_camel_case() {
  assert_eq!(break_camel_case("camelCasing"), "camel Casing");
  assert_eq!(break_camel_case("camelCasingTest"), "camel Casing Test");
}

// https://www.codewars.com/kata/55685cd7ad70877c23000102/train/rust
fn make_negative(n: i32) -> i32 {
  -n.abs()
}

#[test]
fn sample_tests() {
  assert_eq!(make_negative(1), -1);
  assert_eq!(make_negative(-5), -5);
  assert_eq!(make_negative(0), 0);
}

use std::ops::Add;

// https://www.codewars.com/kata/55cbd4ba903825f7970000f5/train/rust
fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
  // solution #1
  match (s1.add(s2).add(s3)) / 3 {
    a if a >= 90 && a <= 100 => 'A',
    a if a >= 80 && a < 90 => 'B',
    a if a >= 70 && a < 80 => 'C',
    a if a >= 60 && a < 70 => 'D',
    _ => 'F',
  }

  // solution #2, experimental feature
  // match (s1 + s2 + s3) / 3 {
  //   n @ 90..=100 => 'A',
  //   n @ 80..90 => 'B',
  //   n @ 70..80 => 'C',
  //   n @ 60..70 => 'D',
  //   _ => 'F'
  // }

  // solution #3, experimental feature
  // match (s1 + s2 + s3) / 3 {
  //   90..=100 => 'A',
  //   80..90 => 'B',
  //   70..80 => 'C',
  //   60..70 => 'D',
  //   _ => 'F'
  // }
}

#[test]
fn test_get_grade() {
  let inner = |s1: u16, s2: u16, s3: u16, expected: char| {
    let actual = get_grade(s1, s2, s3);
    assert_eq!(
      actual, expected,
      "With s1 = {s1}, s2 = {s2}, s = {s3}\nExpected '{expected}' but got '{actual}'"
    )
  };
  inner(100, 100, 100, 'A');
  inner(95, 90, 93, 'A');
  inner(82, 85, 87, 'B');
  inner(60, 82, 76, 'C');
  inner(58, 62, 70, 'D');
  inner(58, 59, 60, 'F');
  inner(0, 0, 0, 'F');
}

use regex::Regex;

// https://www.codewars.com/kata/650a86e8404241005fc744ca/train/rust
fn same_length(txt: &str) -> bool {
  // solution #1
  let mut iter = txt.chars().into_iter();
  let mut item = iter.next();
  let mut c = 0;
  let mut from = "";
  while item.is_some() {
    if item.unwrap() == '1' {
      if from == "-" && c > 0 {
        return false;
      }
      c += 1;
      from = "+";
    } else {
      c -= 1;
      from = "-";
    }
    if c < 0 {
      return false;
    }
    item = iter.next();
  }
  c == 0

  // solution #2, use regex, best practice
  // Regex::new(r"1*0*").unwrap().find_iter(txt).all(|s| {
  //   0 == s
  //     .as_str()
  //     .chars()
  //     .fold(0_i32, |a, c| a + if c == '1' { 1 } else { -1 })
  // })
}

#[test]
fn test_same_length() {
  let inner = |txt: &str, expected: bool| {
    let actual = same_length(txt);
    assert_eq!(
      actual, expected,
      "With txt = \"{txt}\"\nExpected {expected} but got {actual}"
    )
  };
  inner("0", false);
  inner("10", true);
  inner("1010", true);
  inner("1001", false);
  inner("101", false);
  inner("110010", true);
  inner("10010", false);
  inner("110", false);
  inner("11001", false);
  inner("1011100010", true);
  inner("11100011000", false);
  inner("11101010010010", false);
  inner("00110100001111", false);
  inner("1100111000100", false);
  inner("00110011100010", false);
}

use std::collections::HashMap;

// https://www.codewars.com/kata/585d7d5adb20cf33cb000235/train/rust
fn find_uniq(arr: &[f64]) -> f64 {
  // solution #1
  let mut counts: HashMap<String, i32> = HashMap::new();
  arr
    .into_iter()
    .map(|&x| format!("{x:e}"))
    .for_each(|x| *counts.entry(x).or_default() += 1);
  counts
    .iter()
    .find_map(|(k, v)| if *v == 1 { Some(k.clone()) } else { None })
    .unwrap()
    .parse::<f64>()
    .unwrap()

  // solution #2, best practice
  // let x0 = arr[if arr[0] == arr[1] { 0 } else { 2 }];
  // arr.iter().copied().find(|&x| x != x0).unwrap()
}

#[test]
fn test_find_uniq() {
  let inner = |arr: &[f64], expected: f64| {
    assert_eq!(
      find_uniq(arr),
      expected,
      "\nleft is your output. right is expected output.\n input: `{:?}`\n",
      arr
    );
  };
  inner(&[0.0, 1.0, 0.0], 1.0);
  inner(&[1.0, 1.0, 1.0, 2.0, 1.0, 1.0], 2.0);
  inner(&[3.0, 10.0, 3.0, 3.0, 3.0], 10.0);
}
