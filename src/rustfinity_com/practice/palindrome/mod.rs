// https://www.rustfinity.com/practice/rust/challenges/find-the-first-palindrome/description

pub fn find_first_palindrome(start: i32, end: i32) -> Option<i32> {
  fn swap(a: i32, b: i32) -> (i32, i32) {
    if a > b {
      return (b, a);
    }
    (a, b)
  }

  let (start, end) = swap(start, end);

  fn is_palindrome(chars: Vec<char>) -> bool {
    let reversed_chars = chars.iter().cloned().rev().collect::<Vec<_>>();
    let mut index = 0;
    let result = loop {
      if index == chars.len() {
        break true;
      }
      let char = chars[index];
      let reversed_char = reversed_chars[index];
      if char != reversed_char {
        break false;
      }
      index += 1;
    };
    result
  }

  for i in start..=end {
    let chars = i.to_string().chars().collect::<Vec<char>>();
    let x = is_palindrome(chars);
    if x {
      return Some(i);
    }
  }

  None
}
