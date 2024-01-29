

use crate::game::common::*;
use super::Unit;

impl Unit {
  // 能否执行某能力
  pub fn can_skill(&self, skill : Skill) -> bool {
    match skill {
      Skill::Punch => self.can_punch(),
      Skill::Move => self.can_move(),
      Skill::Pass => true,
    }
  }

  pub fn can_skill_list(&self) -> Vec<Skill> {
    let mut list = Vec::new();
    for skl in Skill::iter() {
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

  pub fn can_block(&self, move_dir : Dir) -> bool {
    // 阻挡的条件，可以移动，且不能处于pin或者正在捆绑的状态
    if self.can_move() && !self.pose.is_pin() && !self.pose.is_tieing() {
      if let Some(dir) = self.pose.dir() {
        return dir != move_dir
      } 
    }
    false
  }

  fn can_punch(&self) -> bool {
    // 挥拳的条件，清醒，站立，且上半身没有被捆绑
    self.state.is_able() && self.pose.is_stand() && self.bound.is_upper_able()
  }
  
}

