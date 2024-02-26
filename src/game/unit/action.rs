use crate::game::common::*;
use crate::game::unit::Unit;

impl Unit {
  // 处理回合开始的变动
  pub fn round_start(&mut self) {
    // 眩晕减层
    self.state.stun_restore();
    // 生命恢复
    self.state.hurt_restore();
    // 挣脱捆绑
    if self.can_struggle() && self.bound.need_struggle() && !self.is_defeated() {
      if SHOW_TIE_DETAIL == 1 {
        println!("{} 挣脱捆绑 :", self.name);
      }
      let rope = self.str() * 10;
      self.bound.struggle_main(rope);
      if SHOW_TIE_DETAIL == 1 {
        println!("");
      }
    }
    // 起身（清醒，没被控，束缚状态允许起身）
    if self.state.is_able() && !self.is_ctrled() && self.bound.can_stand() {
      self.pose.stand_exe()
    }
    // 行动状态恢复 (清醒，没被控)
    if self.state.is_able() && !self.is_ctrled() {
      self.state.restore_action();
    }
  }
  
  // 处理角色行动前的变动
  pub fn turn_start(&mut self) {
    self.pose.pin_cancel();
  }
}