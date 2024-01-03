pub mod pawn;

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

  // 索引
  pub fn pawns(&self) -> &[Pawn] {
    &self.pawns
  }
}