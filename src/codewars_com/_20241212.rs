use regex::Regex;
use std::str::FromStr;

// https://www.codewars.com/kata/5556282156230d0e5e000089/rust
fn dna_to_rna(dna: &str) -> String {
  dna.replace("T", "U")
  // dna.chars().into_iter().map(|x| if x == 'T' { 'U' } else { x }).collect()
  // dna.chars().map(|c| match c {
  //   'G' => 'G',
  //   'C' => 'C',
  //   'A' => 'A',
  //   'T' => 'U',
  //   _ => panic!("Invalid DNA character"),
  // }).collect()
}

// https://www.codewars.com/kata/55a2d7ebe362935a210000b2/train/rust
fn find_smallest_int(arr: &[i32]) -> i32 {
  arr.iter().min().unwrap().to_owned()
}

// https://www.codewars.com/kata/5a00e05cc374cb34d100000d/train/rust
fn reverse_seq(n: u32) -> Vec<u32> {
  (1..=n).into_iter().rev().collect()
}

// https://www.codewars.com/kata/5772da22b89313a4d50012f7/train/rust
fn greet(name: &str, owner: &str) -> String {
  format!("Hello {}", if name == owner { "boss" } else { "guest" })
}

// https://www.codewars.com/kata/57a0e5c372292dd76d000d7e/train/rust
fn repeat_str(src: &str, count: usize) -> String {
  src.repeat(count)
}

// https://www.codewars.com/kata/52dbae61ca039685460001ae/train/rust
fn search_for_letters(str: &str) -> String {
  let rgx = Regex::new(r"[a-zA-Z]").unwrap();
  let mut letters: Vec<char> = vec!['0'; 26];
  for ch in str
    .chars()
    .into_iter()
    .filter(|c| rgx.is_match(c.to_string().as_str()))
  {
    match (ch as u8) as usize {
      r @ 65..=90 => letters[r - 65] = '1',
      r @ 97..=122 => letters[r - 97] = '1',
      _ => {}
    }
  }
  letters.into_iter().collect::<String>()
}

// https://www.codewars.com/kata/5bb904724c47249b10000131/train/rust
fn points(games: &[String]) -> u32 {
  let mut point = 0;
  games.iter().for_each(|game| {
    let scores = game
      .split(":")
      .collect::<Vec<&str>>()
      .into_iter()
      .map(|s| s.parse::<u32>().unwrap())
      .collect::<Vec<u32>>();
    point += if scores[0] > scores[1] {
      3
    } else if scores[0] == scores[1] {
      1
    } else {
      0
    };
  });
  point
}

#[cfg(test)]
mod tests {
  use super::{dna_to_rna, find_smallest_int, greet, points, repeat_str, search_for_letters};

  #[test]
  fn dna_to_rna_test() {
    assert_eq!(dna_to_rna("TTTT"), "UUUU");
    assert_eq!(dna_to_rna("GCAT"), "GCAU");
  }

  #[test]
  fn find_smallest_int_tests() {
    assert_eq!(find_smallest_int(&[34, 15, 88, 2]), 2);
    assert_eq!(find_smallest_int(&[34, -345, -1, 100]), -345);
  }

  #[test]
  fn greet_test() {
    assert_eq!(greet("Daniel", "Daniel"), "Hello boss");
    assert_eq!(greet("Greg", "Daniel"), "Hello guest");
  }

  #[test]
  fn repeat_str_test() {
    assert_eq!(repeat_str("a", 4), "aaaa");
    assert_eq!(repeat_str("hello ", 3), "hello hello hello ");
    assert_eq!(repeat_str("abc", 2), "abcabc");
  }

  #[test]
  fn search_for_letters_test() {
    assert_eq!(
      search_for_letters(" cA**& zbZa"),
      "11100000000000000000000001"
    );
  }
  const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

  fn do_fixed_test(e: &[&str], expected: u32) {
    let a = &e.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    assert_eq!(points(a), expected, "{ERR_MSG} with games = {a:?}")
  }

  #[test]
  fn fixed_tests() {
    do_fixed_test(
      &[
        "1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3",
      ],
      30,
    );
    do_fixed_test(
      &[
        "1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4",
      ],
      10,
    );
    do_fixed_test(
      &[
        "0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4",
      ],
      0,
    );
    do_fixed_test(
      &[
        "1:0", "2:0", "3:0", "4:0", "2:1", "1:3", "1:4", "2:3", "2:4", "3:4",
      ],
      15,
    );
    do_fixed_test(
      &[
        "1:0", "2:0", "3:0", "4:4", "2:2", "3:3", "1:4", "2:3", "2:4", "3:4",
      ],
      12,
    );
  }
}
