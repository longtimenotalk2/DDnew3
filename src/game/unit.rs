pub mod pose;
pub mod bound;
pub mod state;
pub mod skill;
pub mod battle;

use pose::Pose;
use bound::Bound;
use state::State;
use crate::game::common::*;

#[derive(Debug, Clone)]
pub struct Unit {
  pub name: String,
  // 基础属性
  str: i32,
  skl: i32,
  spd: i32,
  // 战场状态
  state : State,
  pose : Pose,
  bound : Bound,
}

impl Unit {
  pub fn new(name: &str, str: i32, skl: i32, spd: i32, dir : Dir) -> Unit {
    Unit {
      name: name.to_string(),
      str,
      skl,
      spd,
      state: State::new(),
      pose: Pose::new(dir),
      bound: Bound::new(),
    }
  }
  


  // 定量能力值
  pub fn action_point(&self, fix : i32) -> Option<i32> {
    if self.is_action() {
      let action_point = 100 * self.spd() + fix;
      Some(action_point)
    } else {
      None
    }
  }

  // 属性
  pub fn str(&self) -> i32 {
    (self.str - self.state.attr_decrease()).max(0)
  }

  pub fn skl(&self) -> i32 {
    (self.skl - self.state.attr_decrease()).max(0)
  }

  pub fn spd(&self) -> i32 {
    (self.spd - self.state.attr_decrease()).max(0)
  }
}







