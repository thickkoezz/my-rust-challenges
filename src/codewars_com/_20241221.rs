use itertools::Itertools;
use std::collections::HashMap;

// https://www.codewars.com/kata/57a06b07cf1fa58b2b000252/train/rust
fn is_it_letter(c: char) -> bool {
  c.is_alphabetic()
}

#[test]
fn test_is_it_letter() {
  assert!(is_it_letter('a'));
  assert!(is_it_letter('A'));
  assert!(!is_it_letter('!'));
  assert!(!is_it_letter('1'));
}

// https://www.codewars.com/kata/56a5d994ac971f1ac500003e/train/rust
fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
  // solution #1
  if strarr.len() == 0 || k == 0 || k > strarr.len() {
    return "".to_string();
  }
  let mut vec = vec![];
  let y = strarr.len().wrapping_sub(k);
  for i in 0..=y {
    let x = strarr[i..i + k].join("");
    vec.push(x);
  }
  vec.sort_by_key(String::len);
  let z = vec[vec.len() - 1].len();
  let x = vec.iter().find(|a| a.len() >= z).unwrap();
  x.clone()

  // solution #2, best practice
  // if k > strarr.len() || k == 0 || strarr.len() == 0 {
  //   String::new()
  // } else {
  //   strarr
  //     .windows(k)
  //     .map(|x| x.join(""))
  //     .rev()
  //     .max_by_key(String::len)
  //     .unwrap()
  // }
}

#[test]
fn test_longest_consec() {
  let inner =
    |strarr: Vec<&str>, k: usize, exp: &str| -> () { assert_eq!(&longest_consec(strarr, k), exp) };
  inner(
    vec!["zone", "abigail", "theta", "form", "libe", "zas"],
    2,
    "abigailtheta",
  );
  inner(
    vec![
      "ejjjjmmtthh",
      "zxxuueeg",
      "aanlljrrrxx",
      "dqqqaaabbb",
      "oocccffuucccjjjkkkjyyyeehh",
    ],
    1,
    "oocccffuucccjjjkkkjyyyeehh",
  );
  inner(vec![], 3, "");
  inner(
    vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
    3,
    "ixoyx3452zzzzzzzzzzzz",
  );
  inner(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
  inner(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
  inner(
    vec![
      "llloosssulx",
      "bkkkiiigaaqq",
      "iifffaaad",
      "wwhaa",
      "izzziifpppk",
      "uuufffllll",
      "rrrskkkl",
      "tmfpmmlll",
      "ffftfffdnnn",
    ],
    8,
    "llloosssulxbkkkiiigaaqqiifffaaadwwhaaizzziifpppkuuufffllllrrrskkkltmfpmmlll",
  );
}
