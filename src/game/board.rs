pub mod pawn;
mod action;

use super::unit::Unit;
use crate::game::common::*;
use pawn::Pawn;

pub struct Board {
  pawns : Vec<Pawn>,
}

impl Board {
  pub fn new(data : Vec<(Unit, Team)>) -> Self {
    let mut pawns = Vec::new();
    let mut id : Id = 0;
    for (unit, team) in data {
      pawns.push(Pawn::new(
        unit,
        id,
        team,
      ));
      id += 1;
    }
    Board {
      pawns,
    }
  }

  // 外部函数
  pub fn show(&self) {
    self.show_one_line();
  }

  // 查找
  pub fn id2pos(&self, id : Id) -> Pos {
    for (i, pawn) in self.pawns.iter().enumerate() {
      if pawn.id() == id {
        return i as Pos;
      }
    }
    unreachable!()
  }

  pub fn id2pawn(&self, id : Id) -> &Pawn {
    for pawn in self.pawns.iter() {
      if pawn.id() == id {
        return pawn;
      }
    }
    unreachable!()
  }

  pub fn pos2pawn_try(&self, pos : Pos) -> Option<&Pawn> {
    if pos >= 0 {
      self.pawns.get(pos as usize)
    } else {
      None
    }
  }

  pub fn pos2pawn(&self, pos : Pos) -> &Pawn {
    self.pos2pawn_try(pos).unwrap()
  }

  // 索引
  pub fn pawns(&self) -> &[Pawn] {
    &self.pawns
  }
}