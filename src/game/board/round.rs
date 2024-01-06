

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

pub enum TurnResult {
  Wait,
  End,
}

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

  fn round_next(&mut self) {
    self.round_num += 1;
  }
}

impl Board {
  pub fn main_loop(&mut self) {
    match self.round.phase {
      Phase::Start => self.start(),
      Phase::Main => self.main(),
      Phase::End => {},
    }
  }

  fn start(&mut self) {
    for pawn in self.pawns.iter_mut() {
      pawn.unit_mut().restore_stun();
      pawn.unit_mut().restore_action();
    }
    self.round.team_now = None;
    self.round.ap = None;
    self.round.phase = Phase::Main;
  }

  fn main(&mut self) {
    // 如果还没有当前行动方，则执行主要阶段开始
    if self.round.team_now.is_none() {
      self.main_start();
    }
    // 寻找当前方的可动角色，如果不存在则直接进入结束阶段
    let ids = self.next_can_action_ids();
    if !ids.is_empty() {
      // 处理这些角色的选择（在select mod）
      let can_wait = self.can_wait();
      let selected = self.turn_select(self.round.team_now.unwrap(), &ids, can_wait);
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
    self.round.team_now = Some(1 - self.round.team_now.unwrap());
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
    // 如果当前方尚有可动角色ap大于当前ap，返回对应的值，否则返回None
    if let Some(ap) = self.round.ap {
      if let Some(team_now) = self.round.team_now {
        let mut ids = Vec::new();
        for pawn in self.pawns.iter() {
          if pawn.team() == team_now && pawn.action_point().is_some_and(|a| a > ap) {
            ids.push(pawn.id());
          }
        }
        return ids
      }
    }
    unreachable!()
  }

  fn can_wait(&self) -> bool {
    // 可以等待条件：我方所有可动角色并非都在等待
    let team = self.round.team_now.unwrap();
    for pawn in self.pawns() {
      if pawn.team() == team && pawn.unit().is_action() && !pawn.unit().is_wait(){
        return true;
      }
    }
    false
  }
  
}