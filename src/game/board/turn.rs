

use super::super::board::Board;

use crate::game::common::*;

impl Board {
  pub fn turn_main(&mut self, id : Id, skl : Skill, tgt : Target) {
    let unit = self.id2pawn_mut(id).unit_mut();
    unit.consume_action();

    // 行动前状态变化
    let name = unit.name.clone();
    unit.turn_start();

    match skl {
      Skill::Pass => {
        if SHOW_BATTLE_DETAIL == 1 {
          println!("/n{name} 略过", );
        }
      },
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
        self.melee_exe(id, pos, dir, Skill::Punch);
      },
      Skill::Kick => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.melee_exe(id, pos, dir, Skill::Kick);
      },
      Skill::Tie => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.tie_exe(id, pos, dir);
      },
      Skill::ContinueTie => {
        if SHOW_BATTLE_DETAIL == 1 {
          println!("\n{name} 继续捆绑\n");
        }
      }
      Skill::Untie => {
        let pos = tgt.pos().unwrap();
        let dir = tgt.dir().unwrap();
        self.untie_exe(id, pos, dir);
      },
    }
  }
}