

use super::super::board::Board;

use crate::game::common::*;

impl Board {
  pub fn turn_main(&mut self, id : Id, skl : Skill, tgt : Target) {
    let mut unit = self.id2pawn_mut(id).unit_mut();
    unit.consume_action();

    match skl {
      Skill::Pass => {},
      Skill::Move => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.move_exe(id, pos, dir);
      },
    }
  }
}