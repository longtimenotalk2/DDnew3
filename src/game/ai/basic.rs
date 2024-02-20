use crate::game::board::Board;
use crate::game::common::*;
use crate::game::board::select::*;

impl SelectSet {
  fn first_id(&self) -> Option<Id> {
    for (id, _) in self.data().iter() {
      return Some(*id);
    }
    None
  }
}

pub fn basic_ai(board: &Board, set : &SelectSet) -> Selection {
  // 按角色顺序动，永不等待
  // 优先捆绑没正在被绑的角色，然后优先解绑，然后优先攻击能造成最高输出的敌人，最后略过

  // 自然顺序选择角色
  let id = set.first_id().unwrap();
  let skills = set.id2skills(id);

  // 如果能捆绑，则对没捆好且没正在被捆的角色捆绑
  if skills.contains(&Skill::Tie) {
    let tars = set.skill2targets(id, Skill::Tie);
    for tar in tars {
      let unit = board.pos2pawn(tar.pos().unwrap()).unit();
      if !unit.is_ctrled() && !unit.is_defeated() {
        return Selection::Normal(id, Skill::Tie, tar.clone());
      }
    }
  }

  // 如果能解绑，按顺序
    if skills.contains(&Skill::Untie) {
      let tars = set.skill2targets(id, Skill::Untie);
    for tar in tars {
      return Selection::Normal(id, Skill::Untie, tar.clone());
    }
  }

  // 攻击，只对不倒地的角色攻击，计算每种情况的输出，保留期望最大值
  let mut now : Option<(f64, Skill, Target)> = None;
  let attack = [Skill::Punch, Skill::Kick]; 
  for skill in attack {
      if skills.contains(&skill) {
      let tars = set.skill2targets(id, skill);
      for tar in tars {
        let unit = board.pos2pawn(tar.pos().unwrap()).unit();
        if unit.is_stand() {
          let atk_input = match skill { 
            Skill::Punch => board.id2pawn(id).unit().punch_ability(),
            Skill::Kick => board.id2pawn(id).unit().kick_ability(),
            _ => unreachable!(),
          };
          let analyse = unit.be_attack_analyse(tar.dir().unwrap(), &atk_input);
          let exp = analyse.expect_damage();
          if now.is_none() || now.clone().unwrap().0 < exp {
            now = Some((exp, skill, tar.clone()));
          }
        }
      }
    }
  }
  if let Some((_, skill, tar)) = now {
    return Selection::Normal(id, skill, tar);
  }

  Selection::Normal(id, Skill::Pass, Target::empty())
}