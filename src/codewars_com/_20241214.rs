// https://www.codewars.com/kata/5259b20d6021e9e14c0010d4/train/rust
fn reverse_words(str: &str) -> String {
  str
    .split(" ")
    .map(|w| w.chars().rev().collect())
    .collect::<Vec<String>>()
    .join(" ")
}

#[test]
fn test_reverse_words() {
  assert_eq!(
    reverse_words("double   spaced  words kop"),
    "elbuod   decaps  sdrow pok"
  );
  assert_eq!(
    reverse_words("The quick brown fox jumps over the lazy dog."),
    "ehT kciuq nworb xof spmuj revo eht yzal .god"
  );
  assert_eq!(reverse_words("apple"), "elppa");
  assert_eq!(reverse_words("a b c d"), "a b c d");
}

// https://www.codewars.com/kata/55c45be3b2079eccff00010f/train/rust
fn order(sentence: &str) -> String {
  fn solution_1(sentence: &str) -> String {
    let vec = sentence.split(" ").collect::<Vec<&str>>();
    let mut x = 0;
    let mut res = vec![];
    loop {
      if x == vec.len() {
        break;
      }
      let a = (x + 1).to_string();
      let ch = a.as_str();
      let item = vec.iter().find(|&&x| x.contains(ch));
      if item.is_some() {
        res.push(*item.unwrap());
      }
      x += 1;
    }
    res.join(" ")
  }

  // best practice
  fn solution_2(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
  }

  solution_2(sentence)
}

#[test]
fn test_order() {
  assert_eq!(order("is2 Thi1s T4est 3a"), "Thi1s is2 3a T4est");
  assert_eq!(order(""), "");
}

// https://www.codewars.com/kata/53af2b8861023f1d88000832/train/rust
fn are_you_playing_banjo(name: &str) -> String {
  let v = match name {
    n if n.to_lowercase().starts_with("r") => " plays ",
    _ => " does not play ",
  };
  name.to_owned() + v + "banjo"
}

#[test]
fn test_are_you_playing_banjo() {
  assert_eq!(
    are_you_playing_banjo("Martin"),
    "Martin does not play banjo"
  );
  assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
  assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
  assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
}

// https://www.codewars.com/kata/57f780909f7e8e3183000078/train/rust
fn grow(nums: Vec<i32>) -> i32 {
  nums.into_iter().product()
}

#[test]
fn test_grow() {
  assert_eq!(grow(vec![1, 2, 3]), 6);
  assert_eq!(grow(vec![4, 1, 1, 1, 4]), 16);
  assert_eq!(grow(vec![2, 2, 2, 2, 2, 2]), 64);
}

// https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9/train/rust
fn find_short(s: &str) -> u32 {
  let solution_1 = || {
    let mut ws: Vec<_> = s.split_whitespace().map(String::from).collect();
    ws.into_iter()
      .map(|s| s.len())
      .collect::<Vec<usize>>()
      .into_iter()
      .min()
      .unwrap() as u32
  };

  let solution_2 = || {
    s.split_whitespace()
      .map(|word| word.len())
      .min()
      .unwrap_or(0) as u32
  };

  solution_2()
}

#[test]
fn test_find_short() {
  fn inner(s: &str, expected: u32) {
    let actual = find_short(s);
    assert_eq!(
      actual, expected,
      "With s = \"{s}\"\nExpected {expected} but got {actual}"
    )
  }
  inner("bitcoin take over the world maybe who knows perhaps", 3);
  inner(
    "turns out random test cases are easier than writing out basic ones",
    3,
  );
  inner("lets talk about javascript the best language", 3);
  inner("i want to travel the world writing code one day", 1);
  inner("Lets all go on holiday somewhere very cold", 2);
  inner("Let's travel abroad shall we", 2);
}
