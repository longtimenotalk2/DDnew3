use crate::game::common::*;

#[derive(Debug, Clone, Default)]
pub struct State {
  hurt : i32,
  stun : Option<i32>,
}

impl State {
  pub fn new() -> Self {
    Self::default()
  }

  // 定性状态
  pub fn is_able(&self) -> bool {
    self.stun.is_none()
  }
  

  // 定量影响
  pub fn attr_decrease(&self) -> i32 {
    i2lv(self.hurt).unwrap_or(0)
  }

  // 索引
  pub fn is_stun(&self) -> bool {
    self.stun.is_some()
  }

  pub fn stun_turn(&self) -> i32 {
    self.stun.unwrap_or(0)
  }
    
}