

use super::super::board::Board;

use crate::game::common::*;

impl Board {
  pub fn turn_main(&mut self, id : Id, skl : Skill, tgt : Target) {
    let unit = self.id2pawn_mut(id).unit_mut();
    unit.consume_action();

    // 行动前状态变化
    unit.turn_start();

    match skl {
      Skill::Pass => {},
      Skill::Move => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.move_action_exe(id, pos, dir);
      },
      Skill::MoveTurn => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.move_turn_exe(id, pos, dir);
      },
      Skill::Punch => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.punch_exe(id, pos, dir);
      },
      Skill::Tie => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.tie_exe(id, pos, dir);
      },
      Skill::Untie => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.untie_exe(id, pos, dir);
      },
    }
  }
}