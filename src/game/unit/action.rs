

use super::Unit;

use super::common::*;

impl Unit {
  // 受到攻击分析
  pub fn be_attacked_analyse(&self, input : AttackInput, dir : Dir) -> AttactAnalyse {
    // 信息录入
    let atk = input.atk;
    let acc = input.acc;
    let pir = input.pir;
    let whk = input.whk;
    // 基本防御类数值
    let def = self.def();
    let evd = self.evd(dir);
    let asd = self.asd(dir);
    let rfg = self.rfg(dir);
    // 得出结论
    AttactAnalyse {
      hit : i2pro(acc - evd),
      stt : i2pro(pir - asd),
      cri : i2pro(whk - rfg),
      dmg_asd : ((atk - def) / 2).max(1),
      dmg_stt : (atk - def).max(1),
      dmg_cri : atk.max(1),
    }
  }

  
}