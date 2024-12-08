// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
  // Your code here
  let c1 = s1.trim().chars().count();
  let c2 = s2.trim().chars().count();
  if c1 > c2 {
    Some(s1)
  } else if c1 < c2 {
    return Some(s2);
  } else {
    return None;
  }
}
