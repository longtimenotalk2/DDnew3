use crate::game::board::Board;
use crate::game::common::*;
use crate::game::board::select::*;

pub mod basic;

impl Ai {
  fn select(&self, board: &Board, set : &SelectSet) -> Selection {
    match self {
      Self::Basic => todo!()
    }
  }
}

