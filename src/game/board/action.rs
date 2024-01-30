

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
    let actor = self.id2pawn_mut(id).unit_mut();
    actor.when_tieing();
    let rope = actor.tie_ability();
    let target_id = self.pos2id(pos);
    self.id2pawn_mut(target_id).unit_mut().be_tie_exe(rope);
    
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

  pub fn punch_exe(&mut self, id : Id, pos : Pos, dir : Dir) {
    // 攻击部分
    let atk_input = self.id2pawn(id).unit().punch_ability();
    let analyse = self.pos2pawn(pos).unit().be_attack_analyse(dir, &atk_input);
    let r_hit = self.dice.d100();
    let r_stt = self.dice.d100();
    let r_cri = self.dice.d100();
    let result = AttackResult::from_analyse(&analyse, r_hit, r_stt, r_cri);
    if SHOW_BATTLE_DETAIL == 1 {
      result.show();
    }
    let target_id = self.pos2id(pos);
    self.id2pawn_mut(target_id).unit_mut().be_attack_exe(&result);
    // 移动
    let pos_new = match dir {
      Dir::Left => pos + 1,
      Dir::Right => pos - 1,
    };
    self.move_exe(id, pos_new, dir)
  }
  
  // 移动
  pub fn move_option(&self, id : Id) -> Vec<(Pos, Dir)> {
    let mut list = Vec::new();
    for scan in self.scan(id) {
      if !scan.is_self && scan.can_move {
        list.push((scan.pos, scan.dir.unwrap()))
      }
    }
    list
  }

  pub fn move_exe(&mut self, id : Id, pos : Pos, dir : Dir) {
    let pos_o = self.id2pos(id);
    let mut pawn = self.pawns.remove(pos_o as usize);
    pawn.unit_mut().set_dir(dir);
    let index_new = pos as usize;
    self.pawns.insert(index_new, pawn);
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
  can_move : bool,
  can_touch : bool,
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
      while let Some(pawn) = target {
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
              can_move : !block,
              can_touch : !block,
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
        dir : None,
        is_self : true,
        is_friend : false,
        is_enemy : false,
        can_move : true,
        can_touch : true,
      }
    );
    lists
  }
}