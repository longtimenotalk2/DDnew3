

use crate::game::common::*;
use super::Unit;

impl Unit {
  // 能否执行某能力
  pub fn can_skill(&self, skill : Skill) -> bool {
    match skill {
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

  pub fn can_block(&self) -> bool {
    // 阻挡的条件，可以移动，且不能处于pin或者正在捆绑的状态
    self.can_move() && !self.pose.is_pin() && !self.pose.is_tieing()
  }
  
  pub fn punch_ability(&self) -> Option<AttackInput> {
    // 挥拳的条件，需要上身不被捆绑，且站立
    if !self.bound.is_upper_able() || !self.pose.is_stand() {return None};
    // 攻击力=力量
    let atk = self.str();
    // 命中，技术相关，取决于移动能力
    let acc = ((75. + self.skl() as f64 * 5.) * self.bound.move_coef()) as i32;
    // 穿透，技术相关，取决于移动能力
    let pir = ((150. + self.skl() as f64 * 5.) * self.bound.move_coef()) as i32;
    // 重击，技术相关，取决于移动能力
    let whk = ((50. + self.skl() as f64 * 5.) * self.bound.move_coef()) as i32;
    Some(
      AttackInput {
        atk,
        acc,
        pir,
        whk,
      }
    )
  }
}

