

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

pub struct SelectSet {
  set : HashMap<Id, HashMap<Skill, Vec<Target>>>,
}

impl SelectSet {
  pub fn id2skills(&self, id : Id) -> Vec<Skill> {
    self.set.get(&id).unwrap().keys().cloned().collect()
  }

  pub fn skill2targets(&self, id : Id, skill : Skill) -> Vec<Target> {
    self.set.get(&id).unwrap().get(&skill).unwrap().clone()
  }

  pub fn data(&self) -> &HashMap<Id, HashMap<Skill, Vec<Target>>> {
    &self.set
  }

}

impl Board {
  pub fn turn_select(&self, ids : &[Id], can_wait : bool, use_ai : bool) -> Selection {
    let mut now : (Option<Id>, Option<Skill>, Option<Target>) = (None, None, None);
    let set = self.select_set(ids);
    if use_ai {
      use crate::game::ai::basic;
      return basic::basic_ai(&self, &set)
    }
    loop {
      if let Some(id) = now.0 {
        if let Some(skill) = now.1 {
          // 选择目标/返回上一级
          if let Some(target) = self.choose_target(id, skill, &set) {
            return Selection::Normal(id, skill, target);
          } else {
            now.1 = None;
          }
        } else {
          // 选择技能/返回上一级
          if let Some(skl) = self.choose_skill(id, &set) {
            now.1 = Some(skl);
          } else {
            now.0 = None;
          }
        }
      } else {
        // 选择行动角色/全跳过/等待
        let (pass, wait, id) = self.choose_id(ids, can_wait);
        if pass {return Selection::AllPass;}
        if wait {return Selection::Wait;}
        now = (Some(id.unwrap()), None, None);
      }
    }
    
  }

  fn choose_id(&self, ids : &[Id], can_wait : bool) -> (bool, bool, Option<Id>) {
    // 返回值，是否全跳过，是否等待，选中的id
    let mut options : Vec<String> = ids.iter().map(|id| self.id2pawn(*id).unit().name.clone()).collect();
    options.push("全部略过".to_string());
    if can_wait {
      options.push("等待".to_string());
    }
    let index = io("请选择希望行动的角色：".to_string(), &options, None);
    if index == ids.len() + 1 {
      // 执行了等待
      return (false, true, None);
    } else if index == ids.len() {
      // 执行了全部略过
      return (true, false, None);
    } 
    let id = ids[index];
    return (false, false, Some(id));
  }

  fn choose_skill(&self, id : Id, set : &SelectSet) -> Option<Skill> {
    let skills = set.id2skills(id);
    let mut skills_can = vec!();
    for skill in &skills {
      if set.skill2targets(id, *skill).len() > 0 {
        skills_can.push(*skill);
      }
    }
    Skill::sort(&mut skills_can);
    let mut options : Vec<String> = skills_can.iter().map(|s| s.to_string()).collect();
    options.push("返回上一级".to_string());
    
    let title = format!("{} 选择技能", self.id2pawn(id).unit().name);
    let index = io(title, &options, None);
    if index == options.len() - 1 {
      // 返回上一级
      return None;
    }
    let skill = skills_can[index];
    Some(skill)
  }

  fn choose_target(&self, id : Id, skill : Skill, set : &SelectSet) -> Option<Target> {
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
          let tar = self.pos2pawn(target.pos().unwrap()).unit();
          let pro = if tar.can_anti_ctrl() {100} else {
            let force = self.id2pawn(id).unit().ctrl_ability();
            100 - tar.anti_ctrl_pro(force)
          };
          options.push(format!("{name}, 成功率 {pro} %"));
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
    options.push("返回上一级".to_string());
    let title = format!("{} 为 {} 选择目标", self.id2pawn(id).unit().name, skill.to_string());
    let index = io(title, &options, None);
    if index == options.len() - 1 {
      // 返回上一级
      return None;
    }
    let target = targets[index].clone();
    Some(target)
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