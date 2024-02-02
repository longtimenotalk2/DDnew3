use crate::game::common::*;
use super::super::unit::Unit;

#[derive(Debug, Clone, Default)]
pub struct State {
  action : bool,
  wait : bool,
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
  pub fn is_action(&self) -> bool {
    self.action
  }

  pub fn is_wait(&self) -> bool {
    self.wait
  }
    
  pub fn is_stun(&self) -> bool {
    self.stun.is_some()
  }

  pub fn stun_turn(&self) -> i32 {
    self.stun.unwrap_or(0)
  }

  // 变动
  pub fn restore_action(&mut self) {
    self.action = true;
  }
  
  pub fn hurt_exe(&mut self, hurt : i32) {
    self.hurt += hurt;
  }

  pub fn stun_exe(&mut self, stun : i32) {
    self.action = false;
    if self.stun.is_some() {
      self.stun = Some(self.stun.unwrap() + stun);
    } else {
      self.stun = Some(stun);
    }
  }

  pub fn stun_restore(&mut self) {
    if let Some(mut n) = self.stun {
      n = n - 1;
      if n == 0 {
        self.stun = None;
      } else {
        self.stun = Some(n);
      }
    }
  }

  pub fn hurt_restore(&mut self) {
    // 回复值为受伤开平方
    let heal = (self.hurt as f64).sqrt().floor() as i32;
    self.hurt -= heal;
  }

}

impl Unit {
  pub fn is_action(&self) -> bool {
    self.state.is_action()
  }

  pub fn is_wait(&self) -> bool {
    self.state.is_wait()
  }

  pub fn is_stun(&self) -> bool {
    self.state.is_stun()
  }

  pub fn stun_turn(&self) -> i32 {
    self.state.stun_turn()
  }

  pub fn hurt(&self) -> i32 {
    self.state.hurt
  }

  // 变动
  pub fn consume_action(&mut self) {
    self.state.action = false;
  }

  pub fn to_wait(&mut self) {
    self.state.wait = true;
  }

  pub fn cancel_wait(&mut self) {
    self.state.wait = false;
  }
}