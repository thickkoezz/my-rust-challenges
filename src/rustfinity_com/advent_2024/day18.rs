// https://www.rustfinity.com/practice/rust/challenges/aor-2024-18/description

#[derive(Clone)]
pub struct Sleigh {
  color: String,
  engine: String,
  gift_capacity: u32,
  magical_enhancements: bool,
}

impl Default for Sleigh {
  fn default() -> Self {
    Self {
      color: String::from("red"),
      engine: String::from("reindeer-powered"),
      gift_capacity: 100_u32,
      magical_enhancements: false,
    }
  }
}

#[derive(Clone)]
pub struct SleighBuilder {
  the_sleigh: Sleigh,
}

impl SleighBuilder {
  pub fn new() -> Self {
    Self {
      the_sleigh: Sleigh::default(),
    }
  }

  pub fn color(mut self, color: &str) -> Self {
    self.the_sleigh.color = color.to_string();
    self
  }

  pub fn engine(mut self, engine: &str) -> Self {
    self.the_sleigh.engine = engine.to_string();
    self
  }

  pub fn gift_capacity(mut self, gift_capacity: u32) -> Self {
    self.the_sleigh.gift_capacity = gift_capacity;
    self
  }

  pub fn magical_enhancements(mut self) -> Self {
    self.the_sleigh.magical_enhancements = true;
    self
  }

  pub fn build(self) -> Sleigh {
    self.the_sleigh
  }
}

// Don't Change this implementation
// It is used for the tests
impl Sleigh {
  pub fn color(&self) -> &str {
    &self.color
  }

  pub fn engine(&self) -> &str {
    &self.engine
  }

  pub fn gift_capacity(&self) -> u32 {
    self.gift_capacity
  }

  pub fn magical_enhancements(&self) -> bool {
    self.magical_enhancements
  }
}

pub fn main() {
  let sleigh = SleighBuilder::new()
    .color("gold")
    .engine("magic")
    .gift_capacity(350)
    .magical_enhancements()
    .build();

  assert_eq!(sleigh.color(), "gold");
  assert_eq!(sleigh.engine(), "magic");
  assert_eq!(sleigh.gift_capacity(), 350);
  assert_eq!(sleigh.magical_enhancements(), true);
}
