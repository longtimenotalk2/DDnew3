

// 处理turn中的选择问题
use super::super::board::Board;

use crate::game::common::*;

use crate::game::io::io;

use std::collections::HashMap;

// 选择分3步：选人，选技能，选目标（Option<Pos>）

pub enum Selection {
  Wait,
  AllPass,
  Normal(Id, Skill, Target),
}

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
  pub fn turn_select(&self, ids : &[Id], can_wait : bool) -> Selection {
    let set = self.select_set(ids);
    // 选择角色或者等待
    let mut options : Vec<String> = ids.iter().map(|id| self.id2pawn(*id).unit().name.clone()).collect();
    options.push("全部略过".to_string());
    if can_wait {
      options.push("等待".to_string());
    }
    let index = io("请选择希望行动的角色：".to_string(), &options, None);
    if index == ids.len() + 1 {
      // 执行了等待
      return Selection::Wait;
    } else if index == ids.len() {
      // 执行了全部略过
      return Selection::AllPass;
    } 
    let id = ids[index];

    // 选择技能
    let skills = set.id2skills(id);
    let mut skills_can = vec!();
    for skill in &skills {
      if set.skill2targets(id, *skill).len() > 0 {
        skills_can.push(*skill);
      }
    }
    Skill::sort(&mut skills_can);
    let options : Vec<String> = skills_can.iter().map(|s| s.to_string()).collect();
    
    let title = format!("{} 选择技能", self.id2pawn(id).unit().name);
    let index = io(title, &options, None);
    let skill = skills_can[index];

    // 选择目标
    let targets = set.skill2targets(id, skill);
    let mut options = vec!();
    for target in &targets {
      match skill {
        Skill::Punch => {
          let name = self.pos2pawn(target.pos().unwrap()).unit().name.clone();
          let atk_input = self.id2pawn(id).unit().punch_ability();
          let analyse = self.pos2pawn(target.pos().unwrap()).unit().be_attack_analyse(target.dir().unwrap(), &atk_input);
          let txt = format!("{name} ({})", analyse.to_string());
          options.push(txt);
        }
        Skill::Kick => {
          let name = self.pos2pawn(target.pos().unwrap()).unit().name.clone();
          let atk_input = self.id2pawn(id).unit().kick_ability();
          let analyse = self.pos2pawn(target.pos().unwrap()).unit().be_attack_analyse(target.dir().unwrap(), &atk_input);
          let txt = format!("{name} ({})", analyse.to_string());
          options.push(txt);
        }
        Skill::Tie => {
          let name = self.pos2pawn(target.pos().unwrap()).unit().name.clone();
          options.push(name);
        },
        Skill::Untie => {
          let name = self.pos2pawn(target.pos().unwrap()).unit().name.clone();
          options.push(name);
        },
        Skill::Move => 
      options.push(target.to_string()),
        Skill::MoveTurn => 
      options.push(target.to_string_anti()),
        _ => options.push("无目标".to_string()),
      }
    }
    let title = format!("{} 为 {} 选择目标", self.id2pawn(id).unit().name, skill.to_string());
    let index = io(title, &options, None);
    let target = targets[index].clone();

    Selection::Normal(id, skill, target)
  }
  
  // 根据可动角色，生成完整的SelectSet
  fn select_set(&self, ids : &[Id]) -> SelectSet {
    // 生成所有行动分支
    let mut set : HashMap<Id, HashMap<Skill, Vec<Target>>> = HashMap::new();
    for id in ids {
      let mut sklset : HashMap<Skill, Vec<Target>> = HashMap::new();
      let pawn = self.id2pawn(*id);
      for skl in pawn.unit().can_skill_list() {
        let mut targets = Vec::new();
        match skl {
          Skill::Punch => {
            let can_move = pawn.unit().can_move();
            for (pos, dir) in self.melee_option(*id, can_move) {
              targets.push(Target::new_attack(pos, dir))
            }
          },
          Skill::Kick => {
            let can_move = pawn.unit().can_move();
            for (pos, dir) in self.melee_option(*id, can_move) {
              targets.push(Target::new_attack(pos, dir))
            }
          },
          Skill::Tie => {
            let can_move = pawn.unit().can_move();
            for (pos, dir) in self.tie_option(*id, can_move) {
              targets.push(Target::new_attack(pos, dir))
            }
          },
          Skill::Untie => {
            let can_move = pawn.unit().can_move();
            for (pos, dir) in self.untie_option(*id, can_move) {
              targets.push(Target::new_attack(pos, dir))
            }
          },
          Skill::Move => {
            for (pos, dir) in self.move_option(*id) {
              targets.push(Target::new_move(pos, dir))
            }
          },
          Skill::MoveTurn => {
            for (pos, dir) in self.move_option(*id) {
              targets.push(Target::new_move(pos, dir))
            }
          },
          Skill::Pass => {
            targets.push(Target::empty())
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