use itertools::Itertools;

// https://www.codewars.com/kata/546f922b54af40e1e90001da/train/rust
fn alphabet_position(text: &str) -> String {
  text
    .chars()
    .filter(|&x| x.is_alphabetic())
    .map(|x| (x.to_ascii_lowercase() as u8 - 96).to_string())
    .collect::<Vec<_>>()
    .join(" ")
}

#[test]
fn test_alphabet_position() {
  assert_eq!(alphabet_position("a b c d"), "1 2 3 4".to_string());
  assert_eq!(
    alphabet_position("The sunset sets at twelve o' clock."),
    "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11".to_string()
  );
  assert_eq!(
    alphabet_position("The narwhal bacons at midnight."),
    "20 8 5 14 1 18 23 8 1 12 2 1 3 15 14 19 1 20 13 9 4 14 9 7 8 20".to_string()
  );
}

// https://www.codewars.com/kata/54e6533c92449cc251001667/train/rust
fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
// solution #1
where
  T: IntoIterator,
  T::Item: PartialEq + std::fmt::Debug,
  <T as IntoIterator>::Item: Clone,
{
  let mut y = vec![];
  sequence
    .into_iter()
    .collect::<Vec<_>>()
    .iter()
    .for_each(|x| {
      if y.last() != Option::from(x) {
        y.push(x.clone());
      }
    });
  y
}

// fn unique_in_order<T>(s: T) -> Vec<T::Item> // solution #2, best practice
// where T: IntoIterator, T::Item: PartialEq,
// {
//   s.into_iter().dedup().collect()
// }

#[test]
fn test_unique_in_order() {
  assert_eq!(
    unique_in_order("AAAABBBCCDAABBB".chars()),
    vec!['A', 'B', 'C', 'D', 'A', 'B']
  );
}
