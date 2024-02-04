

use crate::game::common::*;
use super::Unit;

impl Unit {
  
  pub fn punch_ability(&self) -> AttackInput {
    // 攻击力=力量
    let atk = self.str() + 5;
    // 命中，技术相关，取决于移动能力
    let acc = ((75. + self.skl() as f64 * 5.) * self.bound.move_coef()) as i32;
    // 穿透，技术相关，取决于移动能力
    let pir = ((75. + self.skl() as f64 * 5.) * self.bound.move_coef()) as i32;
    // 重击，技术相关，取决于移动能力
    let whk = ((50. + self.skl() as f64 * 2.) * self.bound.move_coef()) as i32;
    
    AttackInput {
      atk,
      acc,
      pir,
      whk,
    }
  }

  pub fn kick_ability(&self) -> AttackInput {
    // 攻击力=力量
    let atk = self.str() + 10;
    // 命中，技术相关，取决于移动能力
    let acc = ((50. + self.skl() as f64 * 5.) * self.bound.move_coef()) as i32;
    // 穿透，技术相关，取决于移动能力
    let pir = ((50. + self.skl() as f64 * 5.) * self.bound.move_coef()) as i32;
    // 重击，技术相关，取决于移动能力
    let whk = ((75. + self.skl() as f64 * 2.) * self.bound.move_coef()) as i32;
    
    AttackInput {
      atk,
      acc,
      pir,
      whk,
    }
  }

  pub fn be_attack_analyse(&self, dir : Dir, input : &AttackInput) -> AttackAnalyse {
    // 命中率 = 精准 - 闪避
    let hit = i2pro(input.acc - self.evd(dir));
    // 直击率 = 穿透 - 格挡
    let stt = i2pro(input.pir - self.asd(dir));
    // 暴击率 = 重击 - 避难
    let cri = i2pro(input.whk - self.rfg(dir));
    // 格挡伤害 = 攻击 - 防御
    let dmg_asd = i2dmg(input.atk - self.def());
    // 直击伤害 = 攻击 - 防御 / 2
    let dmg_stt = i2dmg(input.atk - self.def() / 2);
    // 暴击伤害 = （攻击 - 防御 / 4）
    let dmg_cri = i2dmg(input.atk - self.def() / 4);
    
    AttackAnalyse {hit, stt, cri, dmg_asd, dmg_stt, dmg_cri}
  }

  // &mut 函数
  pub fn be_attack_exe(&mut self, result : &AttackResult, dir : Dir) {
    // 干扰
    self.pose.pin_exe(dir);
    // 造成伤害
    self.state.hurt_exe(result.dmg());
    // 击晕
    if result.is_cri() {
      // 暴击击晕
      let n = self.be_attack_stun_turn(result.dmg());
      if SHOW_BATTLE_DETAIL == 1 {
        println!("击晕 {n} 回合！！");
      }
      self.action_stun(n);
    }
  }

  // 帮助函数

  fn action_stun(&mut self, n : i32) {
    self.state.stun_exe(n);
    self.pose.fall_exe();
  }

  fn def(&self) -> i32 {
    // 防御，力量相关
    self.str()
  }
  
  fn evd(&self, dir : Dir) -> i32 {
    // 闪避，速度相关，取决于反应和移动能力
    if !self.state.is_able() {return 0;}
    (self.spd() as f64 * 5. * self.bound.move_coef() * self.pose.react_coef(dir) * self.pose.move_coef()) as i32
  }

  fn asd(&self, dir : Dir) -> i32 {
    // 格挡，技巧相关，取决于反应以及上半身可用
    if !self.state.is_able() || !self.bound.is_upper_able() {return 0;}
    ((50. + self.skl() as f64 * 5.) * self.pose.react_coef(dir)) as i32
  }

  fn rfg(&self, dir : Dir) -> i32 {
    // 暴击回避，速度相关，取决于反应
    if !self.state.is_able() {return 0;}
    ((50. + self.spd() as f64 * 2.) * self.pose.react_coef(dir)) as i32
  }

  fn stun_resist(&self) -> i32 {
    self.str()
  }

  fn be_attack_stun_turn(&self, dmg : i32) -> i32 {
    if dmg >= self.stun_resist() {
      (dmg - self.stun_resist()) / 10 + 1
    } else {0}
  }

}