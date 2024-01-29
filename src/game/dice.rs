use crate::game::common::Dice;

pub struct DiceOne {}

impl DiceOne {
  pub fn new() -> Self {
    Self {}
  }
}

impl Dice for DiceOne {
  fn d100(&mut self) -> i32 {
    1
  }
}

pub struct Dice50 {}

impl Dice50 {
  pub fn new() -> Self {
    Self {}
  }
}

impl Dice for Dice50 {
  fn d100(&mut self) -> i32 {
    50
  }
}

pub struct DiceWy {
  wy : WyRand
}

impl DiceWy {
  pub fn new(seed : u64) -> Self {
    Self {
      wy: WyRand::new(seed)
    }
  }
}

impl Dice for DiceWy {
  fn d100(&mut self) -> i32 {
    (self.wy.rand() % 100) as i32 + 1
  }
}

struct WyRand {
    state: u64,
}

impl WyRand {
    pub fn new(state: u64) -> Self {
        Self { state }
    }

    pub fn rand(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0xa076_1d64_78bd_642f);
        let t = u128::from(self.state).wrapping_mul(u128::from(self.state ^ 0xe703_7ed1_a0b4_28db));
        (t.wrapping_shr(64) ^ t) as u64
    }
}