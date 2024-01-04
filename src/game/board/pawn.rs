use super::super::unit::Unit;
use crate::game::common::*;

pub struct Pawn {
  unit: Unit,
  _id : Id,
  team : Team,
}

impl Pawn {
  pub fn new(unit: Unit, _id: Id, team: Team) -> Self {
    Self {
      unit,
      _id,
      team,
    }
  }

  pub fn unit(&self) -> &Unit {
    &self.unit
  }

  pub fn team(&self) -> Team {
    self.team
  }
}