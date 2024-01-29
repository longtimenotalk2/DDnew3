use crate::game::common::*;
use crate::game::unit::Unit;

impl Unit {
  // 处理回合开始的变动
  pub fn round_start(&mut self) {
    // 眩晕减层
    self.state.stun_restore();
    // 生命恢复
    self.state.hurt_restore();
    // 行动状态恢复
    self.state.action_restore();
  }
  
  // 处理角色行动前的变动
  pub fn turn_start(&mut self) {
    self.pose.pin_cancel();
  }
}