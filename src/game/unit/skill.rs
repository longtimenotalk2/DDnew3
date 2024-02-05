

use crate::game::common::*;
use super::Unit;

impl Unit {
  // 能否执行某能力
  pub fn can_skill(&self, skill : Skill) -> bool {
    match skill {
      Skill::Punch => self.can_punch(),
      Skill::Kick => self.can_kick(),
      Skill::Tie => self.can_tie(),
      Skill::Untie => self.can_tie(),
      Skill::Move => self.can_move(),
      Skill::MoveTurn => self.can_move(),
      Skill::Pass => true,
    }
  }

  // 是否能执行有意义的行动
  pub fn can_action_sense(&self) -> bool {
    for skl in Skill::iter_sense() {
      if self.can_skill(skl) {
        return true;
      }
    }
    false
  }

  pub fn can_skill_list(&self) -> Vec<Skill> {
    let mut list = Vec::new();
    for skl in Skill::iter_sense() {
      if self.can_skill(skl) {
        list.push(skl);
      }
    }
    list
  } 
  
  pub fn can_move(&self) -> bool {
    // 移动的条件，清醒，站立，下半身没被捆
    self.state.is_able() && self.pose.is_stand() && self.bound.is_lower_able()
  }

  pub fn can_block(&self, _move_dir : Dir) -> bool {
    // 阻挡的条件，可以移动，且不能处于pin或者正在捆绑的状态
    self.can_move() && !self.pose.is_pin() && !self.pose.is_tieing() 
  }

  fn can_punch(&self) -> bool {
    // 挥拳的条件，清醒，站立，且上半身没有被捆绑
    self.state.is_able() && self.pose.is_stand() && self.bound.is_upper_able()
  }

  fn can_kick(&self) -> bool {
    // 踢腿的条件，清醒，站立，且下半身没有被捆绑
    self.state.is_able() && self.pose.is_stand() && self.bound.is_lower_able()
  }

  fn can_tie(&self) -> bool {
    // 捆绑的条件，清醒，站立，且上半身没有被捆绑
    self.state.is_able() && self.pose.is_stand() && self.bound.is_upper_able()
  }

  pub fn can_be_tie(&self) -> bool {
    // 能被绑的条件：处于倒地状态，且有部分没被绑
    if self.bound.is_full() {
      return false;
    }
    !self.pose.is_stand()
  }

  pub fn can_be_untie(&self) -> bool {
    // 能被解绑的条件：有绳索
    self.bound.need_untie()
  }

  pub fn can_struggle(&self) -> bool {
    // 挣脱条件，没晕且没被控
    self.state.is_able() && !self.pose.is_ctrled()
  }

  pub fn can_anti_ctrl(&self) -> bool {
    // 防控条件，没晕
    self.state.is_able()
  }

  pub fn anti_ctrl_pro(&self, force : i32) -> i32 {
    (self.anti_ctrl_ability() - force) * 10 + 100
  }

  pub fn ctrl_ability(&self) -> i32 {
    self.str()
  }

  fn anti_ctrl_ability(&self) -> i32 {
    let mut a = self.str() as f64;
    if !self.bound.is_upper_able() {
      a *= 0.5;
    }
    if !self.bound.is_lower_able() {
      a *= 0.5;
    }
    a.floor() as i32
  }
}

