pub mod pawn;
mod action;
pub mod round;
pub mod turn;
pub mod select;

use super::unit::Unit;
use crate::game::common::*;
use pawn::Pawn;
use round::Round;

pub struct Board {
  pawns : Vec<Pawn>,
  round : Round,
  dice : Box<dyn Dice>,
}

impl Board {
  pub fn new(data : Vec<(Unit, Team)>, dice : impl Dice + 'static) -> Self {
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
      round : Round::new(),
      dice : Box::new(dice),
    }
  }

  // 外部函数
  pub fn show_w_ids(&self, ids : &[Id]) {
    self.show_one_line(ids);
  }

  pub fn play(&mut self) {
    loop {self.main_loop()};
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

  pub fn pos2id(&self, pos : Pos) -> Id {
    self.pos2pawn(pos).id()
  }

  pub fn id2pawn(&self, id : Id) -> &Pawn {
    for pawn in self.pawns.iter() {
      if pawn.id() == id {
        return pawn;
      }
    }
    unreachable!()
  }

  pub fn id2pawn_mut(&mut self, id : Id) -> &mut Pawn {
    for pawn in self.pawns.iter_mut() {
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

  pub fn round(&self) -> &Round {
    &self.round
  }
}