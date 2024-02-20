

use super::super::board::Board;

use crate::game::common::*;

impl Board {
  // 捆绑
  pub fn tie_option(&self, id : Id, can_move : bool) -> Vec<(Pos, Dir)> {
    let mut list = Vec::new();
    for scan in self.scan(id) {
      if scan.is_enemy && scan.can_touch {
        if can_move || scan.dis <= 1 {
          let tar = self.pos2pawn(scan.pos).unit();
          if tar.can_be_tie() {
            list.push((scan.pos, scan.dir.unwrap()))
          }
        }
      }
    }
    list
  }

  pub fn tie_exe(&mut self, id : Id, pos : Pos, dir : Dir) {
    // 取消控制
    self.cancel_ctrl_try(id);
    let target_id = self.pos2id(pos);

    if SHOW_BATTLE_DETAIL == 1 {
      println!("\n{} 捆绑 {}\n", self.id2pawn(id).unit().name, self.id2pawn(target_id).unit().name);
    }
    
    // 如已经被控，解除
    if let Some(id) = self.id2pawn(target_id).unit().ctrled_id() {
      self.cancel_ctrl_force(id);
    }
    let actor = self.id2pawn_mut(id).unit_mut();
    actor.start_tieing_to(target_id);
    
    let tar = self.id2pawn_mut(target_id).unit_mut();
    tar.start_be_tied(id);
    
    // 移动
    let pos_new = match dir {
      Dir::Left => pos + 1,
      Dir::Right => pos - 1,
    };
    self.move_exe(id, pos_new, dir)
  }

  // 解绑
  pub fn untie_option(&self, id : Id, can_move : bool) -> Vec<(Pos, Dir)> {
    let mut list = Vec::new();
    for scan in self.scan(id) {
      if scan.is_friend && scan.can_touch && !scan.is_ctrled {
        if can_move || scan.dis <= 1 {
          let tar = self.pos2pawn(scan.pos).unit();
          if tar.can_be_untie() {
            list.push((scan.pos, scan.dir.unwrap()))
          }
        }
      }
    }
    list
  }

  pub fn untie_exe(&mut self, id : Id, pos : Pos, dir : Dir) {
    if SHOW_BATTLE_DETAIL == 1 {
      println!("\n{} 解绑 {}\n", self.id2pawn(id).unit().name, self.pos2pawn(pos).unit().name);
    }
    
    // 取消控制
    self.cancel_ctrl_try(id);

    // 直接开始解绑
    let rope = self.id2pawn(id).unit().untie_ability();
    self.id2pawn_mut(self.pos2id(pos)).unit_mut().be_untie_exe(rope);
    
    // 移动
    let pos_new = match dir {
      Dir::Left => pos + 1,
      Dir::Right => pos - 1,
    };
    self.move_exe(id, pos_new, dir)
  }
  
  
  // 近战攻击
  pub fn melee_option(&self, id : Id, can_move : bool) -> Vec<(Pos, Dir)> {
    let mut list = Vec::new();
    for scan in self.scan(id) {
      if scan.is_enemy && scan.can_touch {
        if can_move || scan.dis <= 1 {
          list.push((scan.pos, scan.dir.unwrap()))
        }
      }
    }
    list
  }

  pub fn melee_exe(&mut self, id : Id, pos : Pos, dir : Dir, skl : Skill) {
    if SHOW_BATTLE_DETAIL == 1 {
      println!("\n{} {} {}", self.id2pawn(id).unit().name, skl.to_string(), self.pos2pawn(pos).unit().name);
    }
    // 取消控制
    self.cancel_ctrl_try(id);
    // 攻击部分
    
    let atk_input = match skl { 
      Skill::Punch => self.id2pawn(id).unit().punch_ability(),
      Skill::Kick => self.id2pawn(id).unit().kick_ability(),
      _ => unreachable!(),
    };
    let analyse = self.pos2pawn(pos).unit().be_attack_analyse(dir, &atk_input);
    let r_hit = self.dice.d100();
    let r_stt = self.dice.d100();
    let r_cri = self.dice.d100();
    let result = AttackResult::from_analyse(&analyse, r_hit, r_stt, r_cri);
    if SHOW_BATTLE_DETAIL == 1 {
      result.show();
    }
    let target_id = self.pos2id(pos);
    self.id2pawn_mut(target_id).unit_mut().be_attack_exe(&result, dir);
    // 目标失去控制检测
    self.cancel_ctrl_check(target_id);
    // 移动
    let pos_new = match dir {
      Dir::Left => pos + 1,
      Dir::Right => pos - 1,
    };
    self.move_exe(id, pos_new, dir);

    if SHOW_BATTLE_DETAIL == 1 {
      println!("");
    } 
  }
  
  // 移动
  pub fn move_option(&self, id : Id) -> Vec<(Pos, Dir)> {
    let mut list = Vec::new();
    let actor = self.id2pawn(id).unit();
    let tie_id = actor.tieing_id();
    for scan in self.scan(id) {
      if scan.can_move || tie_id.is_some_and (|p| p == self.pos2id(scan.pos)) {
        list.push((scan.pos, scan.dir.unwrap()))
      }
    }
    list
  }

  // 移动行动
  pub fn move_action_exe(&mut self, id : Id, pos : Pos, dir : Dir) {
    // 取消控制
    self.cancel_ctrl_try(id);
    self.move_exe(id, pos, dir)
  }

  pub fn move_turn_exe(self : &mut Self, id : Id, pos : Pos, dir : Dir) {
    // 取消控制
    self.cancel_ctrl_try(id);
    self.move_exe(id, pos, dir);
    // 转身
    self.id2pawn_mut(id).unit_mut().turn_back();
  }

  // 行动附带的移动阶段
  fn move_exe(&mut self, id : Id, pos : Pos, dir : Dir) {
    let tar_id = self.pos2id(pos);
    self.move_pawn(id, pos, Some(dir));
    // 被替换的棋子拉回受控角色
    let tar_unit = self.id2pawn(tar_id).unit();
    let pair_id = if let Some(ctrled_id) = tar_unit.tieing_id() {
      Some(ctrled_id)
    } else if let Some(tieing_id) = tar_unit.ctrled_id() {
      Some(tieing_id)
    } else {
      None
    };
    if let Some(pair_id) = pair_id {
      if (self.id2pos(pair_id) - self.id2pos(tar_id)).abs() > 1 {
        self.move_pawn(tar_id, pos, None);
      }
    }
  }

  // 被动行动
  // 元移动棋子
  fn move_pawn(&mut self, id : Id, pos : Pos, dir : Option<Dir>) {
    let pos_o = self.id2pos(id);
    let mut pawn = self.pawns.remove(pos_o as usize);
    if let Some(dir) = dir {
      pawn.unit_mut().set_dir(dir);
    }
    let index_new = pos as usize;
    self.pawns.insert(index_new, pawn);
  }

  // 取消捆绑尝试
  fn cancel_ctrl_try(&mut self, id : Id) {
    let unit = self.id2pawn(id).unit();
    if unit.is_tieing() {
      self.cancel_ctrl_force(id);
    }
  }

  // 检查是否已经无能力捆绑
  fn cancel_ctrl_check(&mut self, id : Id) {
    let unit = self.id2pawn(id).unit();
    if unit.is_tieing() && !unit.can_skill(Skill::Tie) {
      self.cancel_ctrl_force(id);
    }
  }
  
  pub fn cancel_ctrl_force(&mut self, id : Id) {
    let unit = self.id2pawn_mut(id).unit_mut();
    let id2 = unit.cancel_tieing();
    let unit2 = self.id2pawn_mut(id2).unit_mut();
    unit2.cancel_be_tied();
  }
  
}

// 帮助
#[derive(Debug, Clone)]
pub struct Scan {
  pos : Pos,
  dis : i32,
  dir : Option<Dir>,
  is_self : bool,
  is_friend : bool,
  is_enemy : bool,
  can_move : bool, // 可以移动到此
  can_touch : bool, // 近距离可接触目标
  is_ctrled : bool,  
}

impl Board {
  pub fn scan(&self, id : Id) -> Vec<Scan> {
    // 分别向左和向右看，直到看到不能移动上的位置
    let pos = self.id2pos(id);
    let team = self.id2pawn(id).team();
    let mut lists = [Vec::new(), Vec::new()];
    for i in 0..2 {
      let index = i as usize;
      let q = i * 2 - 1;
      let dir = match i {
        0 => Dir::Left,
        1 => Dir::Right,
        _ => unreachable!(),
      };
      let mut step = 1;
      let mut target = self.pos2pawn_try(pos + step * q);
      let mut block = false;
      let mut ctrl_pair = false;
      while let Some(pawn) = target {
        // 如果处于控制对状态，变化ctrl pair
        if pawn.unit().is_ctrled() || pawn.unit().is_tieing() {
          ctrl_pair = !ctrl_pair;
        }
        let is_ctrled = pawn.unit().is_ctrled();
        if pawn.team() != team && pawn.unit().can_block(dir) {
          // 本目标发生阻挡
          lists[index].push(
            Scan {
              pos : (pos + step * q) as Pos,
              dis : step,
              dir : Some(dir),
              is_self : false,
              is_friend : pawn.team() == team,
              is_enemy : pawn.team() != team,
              can_move : false,
              can_touch : !block,
              is_ctrled,
            }
          );
          block = true;
        } else {
          // 本目标未发生阻挡
          
          lists[index].push(
            Scan {
              pos : (pos + step * q) as Pos,
              dis : step,
              dir : Some(dir),
              is_self : false,
              is_friend : pawn.team() == team,
              is_enemy : pawn.team() != team,
              can_move : !block && !ctrl_pair,
              can_touch : !block,
              is_ctrled,
            }
          );
        }
        step += 1;
        target = self.pos2pawn_try(pos + step * q);
      }
    }

    lists[0].reverse();
    let ind = lists[0].len();
    let mut lists = lists.concat();
    lists.insert(ind, 
      Scan {
        pos : pos,
        dis : 0,
        dir : self.id2pawn(id).unit().dir(),
        is_self : true,
        is_friend : false,
        is_enemy : false,
        can_move : true,
        can_touch : true,
        is_ctrled : false,
      }
    );
    lists
  }
}