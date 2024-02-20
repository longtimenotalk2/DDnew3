

use super::super::board::Board;

use crate::game::common::*;

#[derive(Debug, Clone, Copy)]
pub enum Phase {
  Start,
  Main,
  End,
}

pub struct Round {
  round_num : i32,
  phase : Phase,
  ap : Option<i32>,
  team_now : Option<Team>,
}

// impl Round {
//   pub fn show(&self) {
//     println!("Round: {}", self.round_num);
//     println!("Phase: {:?}", self.phase);
//     println!("AP: {:?}", self.ap);
//     println!("Team now: {:?}", self.team_now);
//   }
// }

impl Round {
  pub fn new() -> Self {
    Round {
      round_num : 0,
      phase : Phase::Start,
      ap : None,
      team_now : None,
    }
  }

  pub fn round_num(&self) -> i32 {
    self.round_num
  }
}

impl Board {
  // 0 0队胜，1 1队胜，2 其它
  pub fn main_loop(&mut self) -> Option<u8> {
    match self.round.phase {
      Phase::Start => self.start(),
      Phase::Main => self.main(),
      Phase::End => {
        return self.end()
      },
    }
    None
  }

  fn start(&mut self) {
    println!("======第 {} 回合开始======", self.round.round_num);
    for pawn in self.pawns.iter_mut() {
      pawn.unit_mut().round_start();
    }
    self.round.team_now = None;
    self.round.ap = None;
    self.round.phase = Phase::Main;
  }

  fn end(&mut self) -> Option<u8> {
    // 反控制
    for i in 0..self.pawns.len() {
      if let Some(tie_id) = self.pawns[i].unit().ctrled_id() { 
        if self.pawns[i].unit().can_anti_ctrl() {
          let force = self.id2pawn(tie_id).unit().ctrl_ability();
          let pro = self.pawns[i].unit().anti_ctrl_pro(force);
          let d100 = self.dice.d100();
          let is = d100 <= pro;
          if is {
            self.cancel_ctrl_force(tie_id)
          }
          if SHOW_BATTLE_DETAIL == 1 {
            let name_1 = &self.pawns[i].unit().name;
            let name_2 = &self.id2pawn(tie_id).unit().name;
            let istxt = if is { "成功" } else { "失败" };
            println!("{name_1} 从 {name_2} 挣脱，{pro}%，{istxt}");
          }
        }
      }
    }
    // 捆绑
    for i in 0..self.pawns.len() {
      let pawn = self.pawns.get(i).unwrap();
      let unit = pawn.unit();
      if let Some(id) = unit.ctrled_id() {
        let rope = self.id2pawn(id).unit().tie_ability();
        let name1 = self.id2pawn(id).unit().name.clone();
        let pawn = self.pawns.get_mut(i).unwrap();
        let unit = pawn.unit_mut();
        let name2 = unit.name.clone();
        if SHOW_BATTLE_DETAIL == 1 {
          println!("{name1} 捆绑 {name2} :", );
        }
        unit.be_tie_exe(rope);
      }
    }
    self.round.phase = Phase::Start;
    self.round.team_now = None;
    self.round.ap = None;
    self.round.round_num += 1;

    // 胜负判定
    let mut t0d = true;
    let mut t1d = true;
    for pawn in &self.pawns {
      let team = pawn.team();
      if !pawn.unit().is_defeated() {
        match team {
          0 => t0d = false,
          1 => t1d = false,
          _ => unreachable!(),
        }
      }
    }
    if t0d && !t1d {
      Some(1)
    } else if !t0d && t1d {
      Some(0)
    } else if t0d && t1d {
      Some(9)
    } else {
      None
    }
  }

  fn main(&mut self) {
    // 如果还没有当前行动方，则执行主要阶段开始
    if self.round.team_now.is_none() {
      self.main_start();
    }
    // 寻找当前方的可动角色，如果不存在则直接进入结束阶段
    let ids = self.next_can_action_ids();
    //self.round.show();
    if !ids.is_empty() {
      // 显示
      self.show_w_ids(&ids);
      // 处理这些角色的选择（在select mod）
      let can_wait = self.can_wait();
      let team_now = self.round.team_now.unwrap();
      let is_ai = match team_now {
        0 => self.team_0_use_ai,
        1 => self.team_1_use_ai,
        _ => unreachable!(),
      };
      let selected = self.turn_select(&ids, can_wait, is_ai);
      use super::select::Selection;
      match selected {
        Selection::Normal(id, skl, tgt) => {
          // 执行技能
          self.turn_main(id, skl, tgt);
          self.after_turn();
        },
        Selection::Wait => {
          // 等待
          for id in &ids {
            self.id2pawn_mut(*id).unit_mut().to_wait();
          }
          self.after_wait();
        },
        Selection::AllPass => {
          // 全部略过
          for id in &ids {
            self.turn_main(*id, Skill::Pass, Target::empty());
          }
          self.after_turn();
        },
      }
    } else {
      // 进入结束阶段
      self.round.phase = Phase::End;
    }
  }

  // 首次进入主要阶段时，调整当前行动方和行动值
  fn main_start(&mut self) {
    // 寻找双方最高的行动值，更高一方先动，更低一方作为当前行动值
    let mut max_aps = [0, 0];
    for pawn in self.pawns.iter() {
      let team = pawn.team();
      let index = team as usize;
      if pawn.action_point().unwrap_or(0) > max_aps[index] { 
        max_aps[index] = pawn.action_point().unwrap_or(0);
      }
    }
    if max_aps[0] > max_aps[1] {
      self.round.team_now = Some(0);
      self.round.ap = Some(max_aps[1]);
    } else {
      self.round.team_now = Some(1);
      self.round.ap = Some(max_aps[0]);
    }
  }

  // 一方一名角色行动后，调整当前行动方和行动值
  fn after_turn(&mut self) {
    // 所有角色等待状态清除
    for pawn in self.pawns.iter_mut() {
      pawn.unit_mut().cancel_wait();
    }

    // 根据对方行动值的最大值调整当前行动值，向更小滑落
    let mut max_ap = 0;
    for pawn in self.pawns.iter() {
      if pawn.team() != self.round.team_now.unwrap() && pawn.action_point().is_some_and(|ap| ap > max_ap) {
        max_ap = pawn.action_point().unwrap();
      }
    }
    if max_ap < self.round.ap.unwrap() {
      self.round.ap = Some(max_ap);
    }

    // 检查该方还有没有比ap大的角色，没有就换边
    for pawn in self.pawns.iter() {
      if pawn.team() == self.round.team_now.unwrap() && pawn.action_point().is_some_and(|ap| ap > self.round.ap.unwrap()) {
        return
      }
    }
    // 换边，并再次计算新的速度值
    self.round.team_now = Some(1 - self.round.team_now.unwrap());
    let mut max_ap = 0;
    for pawn in self.pawns.iter() {
      if pawn.team() != self.round.team_now.unwrap() && pawn.action_point().is_some_and(|ap| ap > max_ap) {
        max_ap = pawn.action_point().unwrap();
      }
    }
    if max_ap < self.round.ap.unwrap() {
      self.round.ap = Some(max_ap);
    }
  }

  // 一方选择等待后
  fn after_wait(&mut self) {
    // 直接更改当前行动方
    self.round.team_now = Some(1 - self.round.team_now.unwrap());
    // 根据对方所有非等待角色行动值的最大值，更新行动值
    let mut max_ap = 0;
    for pawn in self.pawns.iter() {
      if pawn.team() != self.round.team_now.unwrap() && pawn.unit().is_action() && !pawn.unit().is_wait() &&  pawn.action_point().is_some_and(|ap| ap > max_ap) {
        max_ap = pawn.action_point().unwrap();
      }
    }
    if max_ap < self.round.ap.unwrap() {
      self.round.ap = Some(max_ap);
    }
  }

  // ap已设置好之后
  fn next_can_action_ids(&self) -> Vec<Id> {
    // 如果当前方尚有可动角色ap大于等于当前ap，返回对应的值，否则返回None
    if let Some(ap) = self.round.ap {
      if let Some(team_now) = self.round.team_now {
        let mut ids = Vec::new();
        for pawn in self.pawns.iter() {
          if pawn.team() == team_now && pawn.action_point().is_some_and(|a| a >= ap) {
            ids.push(pawn.id());
          }
        }
        return ids
      }
    }
    unreachable!()
  }

  fn can_wait(&self) -> bool {
    // 可以等待条件：我方所有可动角色并非都在等待，且对方有人可动
    let team = self.round.team_now.unwrap();
    
    let mut oppo_has = false;
    let team_o = 1 - team;
    for pawn in self.pawns.iter() {
      if pawn.team() == team_o && pawn.unit().is_action() {
        oppo_has = true;
        break;
      }
    }
    if !oppo_has {return false;}
    
    for pawn in self.pawns() {
      if pawn.team() == team && pawn.unit().is_action() && !pawn.unit().is_wait(){
        return true;
      }
    }
    false
  }
  
}