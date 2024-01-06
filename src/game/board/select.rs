

// 处理turn中的选择问题
use super::super::board::Board;

use crate::game::common::*;

use crate::game::io::io;

use std::collections::HashMap;

// 选择分3步：选人，选技能，选目标（Option<Pos>）



struct SelectSet {
  set : HashMap<Id, HashMap<Skill, Vec<Target>>>,
}

impl SelectSet {
  fn id2skills(&self, id : Id) -> Vec<Skill> {
    self.set.get(&id).unwrap().keys().cloned().collect()
  }

  fn skill2targets(&self, id : Id, skill : Skill) -> Vec<Target> {
    self.set.get(&id).unwrap().get(&skill).unwrap().clone()
  }
    
}

impl Board {
  pub fn turn_select(&self, team : Team, ids : &[Id], can_wait : bool) -> Option<(Id, Skill, Target)> {
    let set = self.select_set(team, ids);
    // 选择角色或者等待
    let mut options : Vec<String> = ids.iter().map(|id| self.id2pawn(*id).unit().name.clone()).collect();
    if can_wait {
      options.push("等待".to_string());
    }
    let index = io("请选择希望行动的角色：".to_string(), &options, None);
    if index == options.len() {
      // 执行了等待
      return None
    }
    let id = ids[index];

    // 选择技能
    let skills = set.id2skills(id);
    let options : Vec<String> = skills.iter().map(|s| s.to_string()).collect();
    let title = format!("{} 选择技能", self.id2pawn(id).unit().name);
    let index = io(title, &options, None);
    let skill = skills[index];

    // 选择目标
    let targets = set.skill2targets(id, skill);
    let options : Vec<String> = targets.iter().map(|t| t.to_string(skill)).collect();
    let title = format!("{} 为 {} 选择目标", self.id2pawn(id).unit().name, skill.to_string());
    let index = io(title, &options, None);
    let target = targets[index].clone();

    Some((id, skill, target))
  }
  
  // 根据可动角色，生成完整的SelectSet
  fn select_set(&self, team : Team, ids : &[Id]) -> SelectSet {
    // 生成所有行动分支
    let mut set : HashMap<Id, HashMap<Skill, Vec<Target>>> = HashMap::new();
    for id in ids {
      let mut sklset : HashMap<Skill, Vec<Target>> = HashMap::new();
      let pawn = self.id2pawn(*id);
      for skl in pawn.unit().can_skill_list() {
        let mut targets = Vec::new();
        match skl {
          Skill::Move => {
            for (pos, dir) in self.move_option(*id) {
              targets.push(Target::new_move(pos, dir))
            }
          },
        }
        sklset.insert(skl, targets);
      }
      set.insert(*id, sklset);
    }
    // 可以等待条件：我方所有角色都处于
    
    SelectSet {
      set,
    }
  }

  

  
}