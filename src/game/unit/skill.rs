

use crate::game::common::*;
use super::Unit;

impl Unit {
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