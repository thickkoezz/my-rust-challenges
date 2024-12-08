// https://www.rustfinity.com/practice/rust/challenges/finite-state-automaton/description

struct Occurrence {
  letter: char,
  advance: bool,
}

struct Pattern {
  occurrences: Vec<Occurrence>,
}

pub trait RemoveFirst<T> {
  fn remove_first(&mut self) -> Option<T>;
}

impl<T> RemoveFirst<T> for Vec<T> {
  fn remove_first(&mut self) -> Option<T> {
    if self.is_empty() {
      return None;
    }
    Some(self.remove(0))
  }
}

pub fn recognize_pattern(input: &str) -> bool {
  let mut patterns: Vec<Pattern> = vec![
    Pattern {
      occurrences: vec![Occurrence {
        letter: 'a',
        advance: true,
      }],
    },
    Pattern {
      occurrences: vec![
        Occurrence {
          letter: 'b',
          advance: false,
        },
        Occurrence {
          letter: 'c',
          advance: true,
        },
      ],
    },
  ];
  let vec = input.to_lowercase().trim().chars().collect::<Vec<_>>();

  let mut exhausted = false;
  for c in vec.into_iter() {
    if let Some(p) = patterns.first() {
      let occurrences = &p.occurrences;
      let mut is_fail = true;
      for o in occurrences.iter() {
        if o.letter == c {
          is_fail = false;
          if o.advance {
            patterns.remove_first();
          }
          break;
        }
      }
      if is_fail {
        break;
      }
    } else {
      exhausted = true;
    }
  }
  if exhausted {
    return false;
  }
  patterns.is_empty()
}

#[test]
fn test() {
  let result = recognize_pattern("abbbc");
  assert_eq!(result, true);

  let result = recognize_pattern("ac");
  assert_eq!(result, true);

  let result = recognize_pattern("abbbd");
  assert_eq!(result, false);

  let result = recognize_pattern("");
  assert_eq!(result, false);

  let result = recognize_pattern("aac");
  assert_eq!(result, false);

  let result = recognize_pattern("bbbc");
  assert_eq!(result, false);

  let result = recognize_pattern("ccc");
  assert_eq!(result, false);

  let result = recognize_pattern("xxxxxaaacc");
  assert_eq!(result, false);

  let result = recognize_pattern("abbbcc");
  assert_eq!(result, false);

  let result = recognize_pattern("abbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbc
abbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbd
abbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbcc");
  assert_eq!(result, false);
}
