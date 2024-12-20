// https://www.rustfinity.com/practice/rust/challenges/aor-2024-19/description

use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

pub struct Sleigh<State> {
  state: PhantomData<State>,
}
impl Sleigh<Empty> {
  pub fn new() -> Self {
    Self { state: PhantomData }
  }
  pub fn load(self) -> Sleigh<Ready> {
    Sleigh { state: PhantomData }
  }
}
impl Sleigh<Ready> {
  pub fn take_off(self) -> Sleigh<Flying> {
    Sleigh { state: PhantomData }
  }
  pub fn unload(self) -> Sleigh<Ready> {
    Sleigh { state: PhantomData }
  }
}
impl Sleigh<Flying> {
  pub fn land(self) -> Sleigh<Ready> {
    Sleigh { state: PhantomData }
  }
}
