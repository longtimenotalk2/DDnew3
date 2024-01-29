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

  // 变动
  fn restore_action(&mut self) {
    if !self.is_stun() {
      self.action = true;
    }
  }

  fn restore_stun(&mut self) {
    if let Some(mut stun) = self.stun {
      stun -= 1;
      if stun <= 0 {
        self.stun = None;
      } else {
        self.stun = Some(stun);
      }
    }
  }

  fn consume_action(&mut self) {
    self.action = false;
    self.wait = false;
  }

  fn to_wait(&mut self) {
    self.wait = true;
  }

  fn cancel_wait(&mut self) {
    self.wait = false;
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
    
}

impl Unit {
  
  // 变动
  pub fn restore_action(&mut self) {
    self.state.restore_action();
  }

  pub fn restore_stun(&mut self) {
    self.state.restore_stun();
  }

  pub fn consume_action(&mut self) {
    self.state.consume_action();
  }

  pub fn to_wait(&mut self) {
    self.state.to_wait();
  }

  pub fn cancel_wait(&mut self) {
    self.state.cancel_wait();
  }

  pub fn hurt_exe(&mut self, hurt : i32) {
    self.state.hurt += hurt;
  }

  pub fn stun_exe(&mut self, stun : i32) {
    self.consume_action();
    if self.state.stun.is_some() {
      self.state.stun = Some(self.state.stun.unwrap() + stun);
    } else {
      self.state.stun = Some(stun);
    }
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
}