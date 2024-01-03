pub mod pose;
pub mod bound;
pub mod state;
pub mod skill;

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
  fn def(&self) -> i32 {
    // 防御，力量相关
    self.str() / 2
  }
  
  fn evd(&self, dir : Dir) -> i32 {
    // 闪避，速度相关，取决于反应和移动能力
    if !self.state.is_able() {return 0;}
    (self.spd() as f64 * 5. * self.bound.move_coef() * self.pose.react_coef(dir) * self.pose.move_coef()) as i32
  }

  fn asd(&self, dir : Dir) -> i32 {
    // 格挡，技巧相关，取决于反应以及上半身可用
    if !self.state.is_able() || !self.bound.is_upper_able() {return 0;}
    ((100. + self.skl() as f64 * 5.) * self.pose.react_coef(dir)) as i32
  }

  fn rfg(&self, dir : Dir) -> i32 {
    // 暴击回避，速度相关，取决于反应
    if !self.state.is_able() {return 0;}
    ((50. + self.spd() as f64 * 5.) * self.pose.react_coef(dir)) as i32
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

  // 基本索引
  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn pose(&self) -> &Pose {
    &self.pose
  }

  pub fn bound(&self) -> &Bound {
    &self.bound
  }
}







