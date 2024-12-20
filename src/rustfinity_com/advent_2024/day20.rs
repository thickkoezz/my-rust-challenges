// https://www.rustfinity.com/practice/rust/challenges/aor-2024-20/result

use std::marker::PhantomData;

pub struct Empty;
pub struct Ready;
pub struct Flying;

pub struct Sleigh<T>
where
  T: State,
{
  pub state: PhantomData<T>,
}

pub trait State {
  fn status() -> &'static str;
}

impl State for Empty {
  fn status() -> &'static str {
    "Empty"
  }
}

impl State for Ready {
  fn status() -> &'static str {
    "Ready"
  }
}

impl State for Flying {
  fn status() -> &'static str {
    "Flying"
  }
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

  pub fn unload(self) -> Sleigh<Empty> {
    Sleigh { state: PhantomData }
  }
}

impl Sleigh<Flying> {
  pub fn land(self) -> Sleigh<Ready> {
    Sleigh { state: PhantomData }
  }
}

impl<T: State> Sleigh<T> {
  pub fn status(&self) -> &'static str {
    T::status()
  }
}
