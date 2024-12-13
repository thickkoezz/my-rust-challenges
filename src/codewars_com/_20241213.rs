// https://www.codewars.com/kata/55f9bca8ecaa9eac7100004a/train/rust
fn past(h: i32, m: i32, s: i32) -> i32 {
  (h * 60 * 60 * 1000) + (m * 60 * 1000) + (s * 1000)
}

#[test]
fn test_past() {
  assert_eq!(past(0, 1, 1), 61000);
  assert_eq!(past(1, 1, 1), 3661000);
  assert_eq!(past(0, 0, 0), 0);
  assert_eq!(past(1, 0, 1), 3601000);
  assert_eq!(past(1, 0, 0), 3600000);
}

// https://www.codewars.com/kata/5769b3802ae6f8e4890009d2/train/rust
fn remove_every_other(arr: &[u8]) -> Vec<u8> {
  arr.iter().step_by(2).map(|&x| x).collect::<Vec<_>>()
}

#[test]
fn test_remove_every_other() {
  assert_eq!(
    remove_every_other(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
    &[1, 3, 5, 7, 9]
  );
}

// https://www.codewars.com/kata/583710ccaa6717322c000105/solutions/rust
fn simple_multiplication(n: u8) -> u8 {
  n * if n % 2 == 0 { 8 } else { 9 }
}

#[test]
fn test_simple_multiplication() {
  assert_eq!(simple_multiplication(1), 9);
  assert_eq!(simple_multiplication(2), 16);
  assert_eq!(simple_multiplication(4), 32);
  assert_eq!(simple_multiplication(5), 45);
}

// https://www.codewars.com/kata/5b077ebdaf15be5c7f000077/train/rust
fn count_sheep(n: u32) -> String {
  (1..=n).map(|x| format!("{} sheep...", x)).collect()
}

#[test]
fn test_count_sheep() {
  assert_eq!(count_sheep(0), "");
  assert_eq!(count_sheep(1), "1 sheep...");
  assert_eq!(count_sheep(2), "1 sheep...2 sheep...");
  assert_eq!(count_sheep(3), "1 sheep...2 sheep...3 sheep...");
}

// https://www.codewars.com/kata/515e271a311df0350d00000f/train/rust
fn square_sum(vec: Vec<i32>) -> i32 {
  vec.iter().fold(0, |sum, &x| sum + x.pow(2))
}

#[test]
fn test_square_sum() {
  assert_eq!(square_sum(vec![1, 2]), 5);
  assert_eq!(square_sum(vec![-1, -2]), 5);
  assert_eq!(square_sum(vec![5, 3, 4]), 50);
  assert_eq!(square_sum(vec![]), 0);
}

// https://www.codewars.com/kata/55d24f55d7dd296eb9000030/train/rust
fn summation(n: i32) -> i32 {
  match n {
    n @ 0..=1 => n,
    _ => n + summation(n - 1),
  }
}

#[test]
fn test_summation() {
  assert_eq!(summation(1), 1);
  assert_eq!(summation(8), 36);
  assert_eq!(summation(22), 253);
  assert_eq!(summation(100), 5050);
  assert_eq!(summation(213), 22791);
}

// https://www.codewars.com/kata/55f2b110f61eb01779000053/train/rust
// use std::cmp::Ordering;
fn get_sum(a: i64, b: i64) -> i64 {
  (a.min(b)..=a.max(b)).sum()
  // match a.cmp(&b) {
  //   Ordering::Equal => a,
  //   Ordering::Greater => (b..=a).into_iter().fold(0, |sum, x| sum + x),
  //   Ordering::Less => (a..=b).into_iter().fold(0, |sum, x| sum + x),
  // }
}

#[test]
fn test_get_sum() {
  assert_eq!(get_sum(0, 1), 1);
  assert_eq!(get_sum(1, 2), 3);
  assert_eq!(get_sum(5, -1), 14);
  assert_eq!(get_sum(505, 4), 127759);
}

// https://www.codewars.com/kata/54edbc7200b811e956000556/train/rust
fn count_sheep_2(sheep: &[bool]) -> u8 {
  // sheep.iter().fold(0, |sum, &x| if x { sum + 1 } else { sum })
  sheep.iter().filter(|x| **x).count() as u8
}

#[test]
fn test_count_sheep_2() {
  assert_eq!(count_sheep_2(&[false]), 0);
  assert_eq!(count_sheep_2(&[true]), 1);
  assert_eq!(count_sheep_2(&[true, false]), 1);
}

// https://www.codewars.com/kata/5583090cbe83f4fd8c000051/train/rust
fn digitize(n: u64) -> Vec<u8> {
  format!("{}", n)
    .chars()
    .map(|c| c.to_digit(10).unwrap() as u8)
    .rev()
    .collect()
}

#[test]
fn test_digitize() {
  assert_eq!(digitize(348597), vec![7, 9, 5, 8, 4, 3]);
  assert_eq!(digitize(35231), vec![1, 3, 2, 5, 3]);
  assert_eq!(digitize(0), vec![0]);
}

// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/rust
pub fn remove_char(s: &str) -> String {
  s[1..s.len() - 1].to_string()
}

#[test]
fn test_remove_char() {
  assert_eq!(remove_char("eloquent"), "loquen");
  assert_eq!(remove_char("country"), "ountr");
  assert_eq!(remove_char("person"), "erso");
  assert_eq!(remove_char("place"), "lac");
  assert_eq!(remove_char("ok"), "");
  assert_eq!(remove_char("ooopsss"), "oopss");
}

// https://www.codewars.com/kata/57a429e253ba3381850000fb/train/rust
fn bmi(weight: u32, height: f32) -> &'static str {
  match weight as f32 / height.powf(2f32) {
    r if r <= 18.5 => "Underweight",
    r if r <= 25.0 => "Normal",
    r if r <= 30.0 => "Overweight",
    _ => "Obese",
  }
}

#[test]
fn test_bmi() {
  assert_eq!(bmi(50, 1.80), "Underweight");
  assert_eq!(bmi(80, 1.80), "Normal");
  assert_eq!(bmi(90, 1.80), "Overweight");
  assert_eq!(bmi(110, 1.80), "Obese");
}

// https://www.codewars.com/kata/5502c9e7b3216ec63c0001aa/train/rust
fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
  data
    .iter()
    .map(|&(x, y)| {
      if x < 55 || y <= 7 {
        "Open".to_string()
      } else {
        "Senior".to_string()
      }
    })
    .collect::<Vec<_>>()
}

#[test]
fn test_open_or_senior() {
  assert_eq!(
    open_or_senior(vec![(45, 12), (55, 21), (19, -2), (104, 20)]),
    vec!["Open", "Senior", "Open", "Senior"]
  );
  assert_eq!(
    open_or_senior(vec![(3, 12), (55, 1), (91, -2), (54, 23)]),
    vec!["Open", "Open", "Open", "Open"]
  );
}

// https://www.codewars.com/kata/55f8a9c06c018a0d6e000132/train/rust
fn validate_pin(pin: &str) -> bool {
  (pin.len() == 4 || pin.len() == 6) && pin.chars().all(|c| c.is_digit(10))
}

#[test]
fn test_validate_pin() {
  assert_eq!(validate_pin("1234"), true);
  assert_eq!(validate_pin("0000"), true);
  assert_eq!(validate_pin("1111"), true);
  assert_eq!(validate_pin("123456"), true);
  assert_eq!(validate_pin("098765"), true);
  assert_eq!(validate_pin("000000"), true);
  assert_eq!(validate_pin("123456"), true);
  assert_eq!(validate_pin("090909"), true);
  assert_eq!(validate_pin("09090999"), false);
  assert_eq!(validate_pin("abcdef"), false);
}

// https://www.codewars.com/kata/576bb71bbbcf0951d5000044/train/rust
fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
  match input.len() {
    0 => vec![],
    _ => {
      vec![
        input.iter().filter(|&&x| x > 0).count() as i32,
        input.iter().filter(|&&x| x < 0).sum(),
      ]
    }
  }
}

#[test]
fn test_count_positives_sum_negatives() {
  fn inner(a: &[i32], expected: &[i32]) {
    let actual = count_positives_sum_negatives(a.to_vec());
    assert_eq!(
      actual, expected,
      "With input = {a:?}\nExpected {expected:?} but got {actual:?}"
    )
  }
  inner(
    &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15],
    &[10, -65],
  );
  inner(&[], &[]);
  inner(
    &[0, 2, 3, 0, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14],
    &[8, -50],
  );
  inner(&[0, 1, 2, 3, 4, 5], &[5, 0]);
  inner(&[1, 2, 3, 4, 5], &[5, 0]);
  inner(&[0, -1, -2, -3, -4, -5], &[0, -15]);
  inner(&[-1, -2, -3, -4, -5], &[0, -15]);
  inner(&[0, 0, 0, 0], &[0, 0]);
  inner(&[0], &[0, 0]);
}

// https://www.codewars.com/kata/57a0556c7cb1f31ab3000ad7/train/rust
fn make_upper_case(s: &str) -> String {
  s.to_uppercase()
}

#[test]
fn test_make_upper_case() {
  assert_eq!(make_upper_case("hello"), "HELLO");
}

// https://www.codewars.com/kata/582cb0224e56e068d800003c/train/rust
fn litres(time: f64) -> i32 {
  (time * 0.5).floor() as i32
}

#[test]
fn sample_tests() {
  assert_eq!(litres(2.), 1);
  assert_eq!(litres(1.4), 0);
  assert_eq!(litres(12.3), 6);
  assert_eq!(litres(0.82), 0);
  assert_eq!(litres(11.8), 5);
  assert_eq!(litres(1787.), 893);
  assert_eq!(litres(0.), 0);
}

// https://www.codewars.com/kata/57a1d5ef7cb1f3db590002af/train/rust
fn fibonacci(n: u32) -> u32 {
  match n {
    n if n < 2 => n,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

#[test]
fn test_fibonacci() {
  fn inner(n: u32, expected: u32) {
    let msg = "\nYour result (left) did not match the expected output (right)";
    assert_eq!(fibonacci(n), expected, "{msg} with n = {n}")
  }
  inner(0, 0);
  inner(1, 1);
  inner(2, 1);
  inner(3, 2);
  inner(4, 3);
  inner(5, 5);
}

// https://www.codewars.com/kata/5467e4d82edf8bbf40000155/train/rust
fn descending_order(x: u64) -> u64 {
  let mut vec = x.to_string().chars().collect::<Vec<char>>();
  vec.sort_by(|a, b| b.cmp(a));
  vec.into_iter().collect::<String>().parse::<u64>().unwrap()
}

#[test]
fn test_descending_order() {
  assert_eq!(descending_order(0), 0);
  assert_eq!(descending_order(1), 1);
  assert_eq!(descending_order(15), 51);
  assert_eq!(descending_order(1021), 2110);
  assert_eq!(descending_order(123456789), 987654321);
  assert_eq!(descending_order(145263), 654321);
  assert_eq!(descending_order(1254859723), 9875543221);
}

// https://www.codewars.com/kata/554e4a2f232cdd87d9000038/train/rust
fn dna_strand(dna: &str) -> String {
  dna
    .chars()
    .into_iter()
    .map(|ch| match ch {
      'A' => 'T',
      'T' => 'A',
      'C' => 'G',
      'G' => 'C',
      _ => ch,
    })
    .collect::<String>()
}

#[test]
fn fixed_dna_strand() {
  fn inner(s: &str, expected: &str) {
    let actual = dna_strand(s);
    assert_eq!(
      actual, expected,
      "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\""
    )
  }
  inner("AAAA", "TTTT");
  inner("ATTGC", "TAACG");
  inner("GTAT", "CATA");
}

// https://www.codewars.com/kata/56606694ec01347ce800001b/train/rust
fn is_triangle(a: i64, b: i64, c: i64) -> bool {
  (a + b + c) as f64 / 2f64 > a.max(b).max(c) as f64
}

#[test]
fn test_is_triangle() {
  assert_eq!(is_triangle(1, 2, 2), true,);
  assert_eq!(is_triangle(7, 2, 2), false);
  assert_eq!(is_triangle(1, 2, 3), false);
  assert_eq!(is_triangle(1, 3, 2), false);
  assert_eq!(is_triangle(3, 1, 2), false);
  assert_eq!(is_triangle(5, 1, 2), false);
  assert_eq!(is_triangle(1, 2, 5), false);
  assert_eq!(is_triangle(2, 5, 1), false);
  assert_eq!(is_triangle(4, 2, 3), true);
  assert_eq!(is_triangle(5, 1, 5), true);
  assert_eq!(is_triangle(2, 2, 2), true);
  assert_eq!(is_triangle(-1, 2, 3), false);
  assert_eq!(is_triangle(1, -2, 3), false);
  assert_eq!(is_triangle(1, 2, -3), false);
  assert_eq!(is_triangle(0, 2, 3), false);
}

// https://www.codewars.com/kata/57eadb7ecd143f4c9c0000a3/train/rust
fn abbrev_name(name: &str) -> String {
  name
    .split(" ")
    .collect::<Vec<&str>>()
    .iter()
    .map(|&x| {
      x.chars()
        .into_iter()
        .take(1)
        .map(|x| x.to_uppercase().to_string())
        .collect()
    })
    .collect::<Vec<String>>()
    .join(".")
}

#[test]
fn test_abbrev_name() {
  assert_eq!(abbrev_name("Sam Harris"), "S.H");
  assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
  assert_eq!(abbrev_name("Evan Cole"), "E.C");
  assert_eq!(abbrev_name("P Favuzzi"), "P.F");
  assert_eq!(abbrev_name("David Mendieta"), "D.M");
}

// https://www.codewars.com/kata/559590633066759614000063/train/rust
fn min_max(lst: &[i32]) -> (i32, i32) {
  (
    lst.iter().min().map(|&x| x).unwrap(),
    lst.iter().max().map(|&x| x).unwrap(),
  )
}

#[test]
fn test_min_max() {
  fn inner(arr: &[i32], expected: (i32, i32)) {
    let msg = "\nYour result (left) did not match the expected output (right)";
    assert_eq!(min_max(arr), expected, "{msg} with lst = {arr:?}")
  }
  for (arr, expected) in [
    (vec![1, 2, 3, 4, 5], (1, 5)),
    (vec![2334454, 5], (5, 2334454)),
  ] {
    inner(&arr, expected)
  }
}

fn find_next_square(sq: u64) -> Option<u64> {
  match ((sq as f64).sqrt()) % 1f64 {
    0f64 => Option::from(((((sq as f64).sqrt()) + 1f64) as u64).pow(2)),
    _ => None,
  }
}

#[test]
fn test_find_next_square() {
  fn inner(n: u64, expected: Option<u64>) {
    let actual = find_next_square(n);
    assert_eq!(
      actual, expected,
      "\nYour result (left), did not match the correct answer (right) for n = {n}"
    );
  }
  inner(121, Some(144));
  inner(625, Some(676));
  inner(319_225, Some(320_356));
  inner(15_241_383_936, Some(15_241_630_849));
  inner(155, None);
  inner(342_786_627, None);
}

fn positive_sum(slice: &[i32]) -> i32 {
  slice.iter().filter(|&&x| x > 0).sum()
}

#[test]
fn test_positive_sum() {
  assert_eq!(positive_sum(&[1, 2, 3, 4, 5]), 15);
  assert_eq!(positive_sum(&[1, -2, 3, 4, 5]), 13);
  assert_eq!(positive_sum(&[-1, 2, 3, 4, -5]), 9);
  assert_eq!(positive_sum(&[]), 0);
  assert_eq!(positive_sum(&[-1, -2, -3, -4, -5]), 0);
}

// https://www.codewars.com/kata/5a3fe3dde1ce0e8ed6000097/train/rust
fn century(year: u32) -> u32 {
  (year as f64 / 100f64).ceil() as u32
}

#[test]
fn test_century() {
  fn inner(year: u32, expected: u32) {
    let actual = century(year);
    assert_eq!(
      actual, expected,
      "\n\nFor year = {}\n expected: {}\n actual: {}",
      year, expected, actual
    );
  }
  inner(1905, 20);
  inner(1700, 17);
  inner(89, 1);
  inner(100, 1);
  inner(101, 2);
}
