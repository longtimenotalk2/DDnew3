

use super::super::board::Board;

use crate::game::common::*;

impl Board {
  // 移动
  pub fn move_option(&self, id : Id) -> Vec<(Pos, Dir)> {
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
      while let Some(pawn) = target {
        if pawn.team() != team && pawn.unit().can_block() {
          // 发生阻挡
          target = None;
        } else {
          // 未发生阻挡
          lists[index].push(((pos + step * q) as Pos, dir));
          step += 1;
          target = self.pos2pawn_try(pos + step * q);
        }
      }
    }

    lists[0].reverse();
    lists.concat()
  }

  pub fn move_exe(&mut self, id : Id, pos : Pos, dir : Dir) {
    let pos_o = self.id2pos(id);
    let mut pawn = self.pawns.remove(pos_o as usize);
    pawn.unit_mut().set_dir(dir);
    let index_new = pos as usize;
    self.pawns.insert(index_new, pawn);
  }
}