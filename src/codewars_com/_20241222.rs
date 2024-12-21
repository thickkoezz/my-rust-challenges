use itertools::Itertools;

// https://www.codewars.com/kata/54da5a58ea159efa38000836/train/rust
fn find_odd(arr: &[i32]) -> i32 {
  // solution #1
  **arr
    .into_iter()
    .counts()
    .iter()
    .find(|&(&&k, &v)| v % 2 > 0)
    .unwrap()
    .0

  // solution #2, best practice
  // arr.iter().fold(0, |a,v| a^v)
}

#[test]
fn test_find_odd() {
  assert_eq!(
    find_odd(&vec![
      20, 1, -1, 2, -2, 3, 3, 5, 5, 1, 2, 4, 20, 4, -1, -2, 5
    ]),
    5
  );
  assert_eq!(find_odd(&vec![1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5]), -1);
  assert_eq!(find_odd(&vec![20, 1, 1, 2, 2, 3, 3, 5, 5, 4, 20, 4, 5]), 5);
  assert_eq!(find_odd(&vec![10]), 10);
  assert_eq!(find_odd(&vec![1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1]), 10);
  assert_eq!(find_odd(&vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 10, 10]), 1);
}

// https://www.codewars.com/kata/5667e8f4e3f572a8f2000039/train/rust
fn accum(s: &str) -> String {
  s.chars()
    .into_iter()
    .enumerate()
    .map(|(i, c)| {
      format!(
        "{}{}",
        c.to_uppercase(),
        c.to_lowercase().to_string().repeat(i)
      )
    })
    .collect::<Vec<String>>()
    .join("-")
}

#[test]
fn test_accum() {
  assert_eq!(
    accum("ZpglnRxqenU"),
    "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
  );
  assert_eq!(
    accum("NyffsGeyylB"),
    "N-Yy-Fff-Ffff-Sssss-Gggggg-Eeeeeee-Yyyyyyyy-Yyyyyyyyy-Llllllllll-Bbbbbbbbbbb"
  );
  assert_eq!(
    accum("MjtkuBovqrU"),
    "M-Jj-Ttt-Kkkk-Uuuuu-Bbbbbb-Ooooooo-Vvvvvvvv-Qqqqqqqqq-Rrrrrrrrrr-Uuuuuuuuuuu"
  );
  assert_eq!(
    accum("EvidjUnokmM"),
    "E-Vv-Iii-Dddd-Jjjjj-Uuuuuu-Nnnnnnn-Oooooooo-Kkkkkkkkk-Mmmmmmmmmm-Mmmmmmmmmmm"
  );
  assert_eq!(
    accum("HbideVbxncC"),
    "H-Bb-Iii-Dddd-Eeeee-Vvvvvv-Bbbbbbb-Xxxxxxxx-Nnnnnnnnn-Cccccccccc-Ccccccccccc"
  );
}

// https://www.codewars.com/kata/56747fd5cb988479af000028/train/rust
fn get_middle(s: &str) -> &str {
  // solution #1
  let l = s.len() / 2;
  match s.len() % 2 {
    0 => &s[l - 1..=l],
    _ => &s[l..=l],
  }

  // solution #2, best practice
  // &s[(s.len()-1)/2..s.len()/2+1]
}

#[test]
fn test_middle() {
  assert_eq!(get_middle("test"), "es");
  assert_eq!(get_middle("testing"), "t");
  assert_eq!(get_middle("middle"), "dd");
  assert_eq!(get_middle("A"), "A");
  assert_eq!(get_middle("of"), "of");
}

// https://www.codewars.com/kata/58649884a1659ed6cb000072/train/rust
fn update_light(current: &str) -> String {
  match current {
    "green" => "yellow".to_string(),
    "yellow" => "red".to_string(),
    _ => "green".to_string(),
  }
}

#[test]
fn test_update_light() {
  assert_eq!(update_light("green"), "yellow");
  assert_eq!(update_light("yellow"), "red");
  assert_eq!(update_light("red"), "green");
}

// https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c/train/rust
fn max_sequence(seq: &[i32]) -> i32 {
  // solution #1
  let mut max = 0;
  for i in 1..=seq.len() {
    let mut iter = seq.windows(i);
    let mut item = iter.next();
    while item.is_some() {
      let item2 = item.unwrap();
      let max1 = item2.iter().map(|&i| i).sum::<i32>();
      max = i32::max(max1, max);
      item = iter.next();
    }
  }
  max

  // solution #2, good practice
  // seq
  //   .iter()
  //   .scan(0, |s, x| {
  //     *s = (*s + x).max(0);
  //     Some(*s)
  //   })
  //   .max()
  //   .unwrap_or(0)

  // solution #3, good practice
  // let mut max = 0;
  // seq.iter().fold(0, |prev, &v|{
  //   let p = v.max(prev + v);
  //   max = max.max(p);
  //   p
  // });
  // max

  // solution #4, best practice
  // (1..=seq.len())
  //   .flat_map(|i| seq.windows(i).map(|s| s.iter().sum()))
  //   .chain(Some(0))
  //   .max()
  //   .unwrap_or(0)

  // solution #5, use itertools, good practice
  // itertools::iproduct!(0..=seq.len(), 0..=seq.len())
  //   .filter(|(i, j)| i <= j)
  //   .map(|(i, j)| &seq[i..j])
  //   .map(|subseq| subseq.iter().sum())
  //   .max()
  //   .unwrap()

  // solution #6, good practice
  // seq.iter().fold((0i32, 0i32), |mut acc, &x| {
  //   acc.1 = (acc.1 + x).max(0);
  //   acc.0 = acc.0.max(acc.1);
  //   acc
  // }).0

  // solution #7, good practice
  // (1..=seq.len())
  //   .map(|sz| {
  //     seq.windows(sz)
  //       .map(|part| part.iter().fold(0, |sum, i| sum + i))
  //       .fold(0, i32::max)
  //   })
  //   .fold(0, i32::max)
}

#[test]
fn test_max_sequence() {
  assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
  assert_eq!(max_sequence(&[11]), 11);
  assert_eq!(max_sequence(&[-32]), 0);
}

// https://www.codewars.com/kata/57a083a57cb1f31db7000028/train/rust
fn powers_of_two(n: u8) -> Vec<u128> {
  (0..=n).map(|i| u128::pow(2, i as u32)).collect()
}

#[test]
fn test_powers_of_two() {
  let inner = |n: u8, expected: &[u128]| {
    let actual = powers_of_two(n);
    assert_eq!(
      actual, expected,
      "With n = {n}\nExpected {expected:?}\nBut got {actual:?}"
    )
  };
  inner(0, &[1]);
  inner(1, &[1, 2]);
  inner(4, &[1, 2, 4, 8, 16]);
}
