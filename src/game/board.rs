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
  team_0_use_ai : bool,
  team_1_use_ai : bool,
  
}

impl Board {
  pub fn new(dice : impl Dice + 'static, ai0 : bool, ai1 : bool, round_limit : Option<i32>) -> Self {
    
    Board {
      pawns : vec!(),
      round : Round::new(round_limit),
      dice : Box::new(dice),
      team_0_use_ai : ai0,
      team_1_use_ai : ai1,
    }
  }

  pub fn insert_unit(&mut self, unit : Unit, team : Team) {
    let index = self.pawns.len() as u32;
    self.pawns.push(Pawn::new(
      unit,
      index,
      team,
    ));
  }

  // 外部函数
  pub fn show_w_ids(&self, ids : &[Id]) {
    self.show_one_line(ids);
  }

  pub fn play(&mut self) -> PlayResult {
    loop {
      let r = self.main_loop();
      if let Some(r) = r {
        return r;
      }
    };
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