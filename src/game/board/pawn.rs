use super::super::unit::Unit;
use crate::game::common::*;

pub struct Pawn {
  unit: Unit,
  id : Id,
  team : Team,
}

impl Pawn {
  pub fn new(unit: Unit, id: Id, team: Team) -> Self {
    Self {
      unit,
      id,
      team,
    }
  }

  // 属性
  pub fn action_point(&self) -> Option<i32> {
    let fix = self.id as i32;
    self.unit.action_point(fix)
  }

  // 索引

  pub fn unit(&self) -> &Unit {
    &self.unit
  }

  pub fn team(&self) -> Team {
    self.team
  }

  pub fn id(&self) -> Id {
    self.id
  }

  pub fn unit_mut(&mut self) -> &mut Unit {
    &mut self.unit
  }
}