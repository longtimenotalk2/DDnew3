use crate::game::unit::Unit;
use crate::game::common::*;

#[derive(Debug, Clone, Default)]
pub struct Bound {
  wrist : bool,
  arm : bool,
  leg : bool,
  lock : bool,
  wrist_process : i32,
  arm_process : i32,
  leg_process : i32,
  lock_process : i32,
  wrist_tightness : i32,
  arm_tightness : i32,
  leg_tightness : i32,
  lock_tightness : i32,
}

impl Bound {
  pub fn new() -> Self {
    Self::default()
  }

  // 定性影响
  pub fn is_upper_able(&self) -> bool {
    !self.wrist
  }

  pub fn is_lower_able(&self) -> bool {
    !self.leg
  }

  // 定量影响
  pub fn move_coef(&self) -> f64 {
    let mut c = 1.0;
    if self.wrist {c *= 0.8}
    if self.arm {c *= 0.8}
    if self.leg {c *= 0.5}
    if self.lock {c *= 0.0}
    c
  }

  // 索引

  fn part_data(&self, part : BoundPart) -> (bool, i32, i32) {
    match part {
      BoundPart::Wrist => (self.wrist, self.wrist_process, self.wrist_tightness),
      BoundPart::Arm => (self.arm, self.arm_process, self.arm_tightness),
      BoundPart::Leg => (self.leg, self.leg_process, self.leg_tightness),
      BoundPart::Lock => (self.lock, self.lock_process, self.lock_tightness),
    }
  }

  fn part_is_mut(&mut self, part : BoundPart) -> &mut bool {
    match part {
      BoundPart::Wrist => &mut self.wrist,
      BoundPart::Arm => &mut self.arm,
      BoundPart::Leg => &mut self.leg,
      BoundPart::Lock => &mut self.lock,
    }
  }

  fn part_tightness_mut(&mut self, part : BoundPart) -> &mut i32 {
    match part {
      BoundPart::Wrist => &mut self.wrist_tightness,
      BoundPart::Arm => &mut self.arm_tightness,
      BoundPart::Leg => &mut self.leg_tightness,
      BoundPart::Lock => &mut self.lock_tightness,
    }
  }

  fn part_process_mut(&mut self, part : BoundPart) -> &mut i32 {
    match part {
      BoundPart::Wrist => &mut self.wrist_process,
      BoundPart::Arm => &mut self.arm_process,
      BoundPart::Leg => &mut self.leg_process,
      BoundPart::Lock => &mut self.lock_process,
    }
  }

  fn part_state(&self, part : BoundPart) -> BoundState {
    let (is, process, tightness) = self.part_data(part);
    state(is, process, tightness)
  }

  pub fn is_full(&self) -> bool {
    for part in BoundPart::tie_order() {
      if self.part_state(part) != BoundState::Full {
        return false;
      }
    }
    true
  }

  // 变动
  fn tie_part(&mut self, part : BoundPart, rope: &mut i32) {
    use BoundState::*;
    match self.part_state(part) {
      Full => (),
      Loose => {
        let t = self.part_tightness_mut(part);
        let start = *t;
        *t += *rope;
        if *t > 100 {
          let remain = *t - 100;
          *t = 100;
          *rope = remain;
        } else {
          *rope = 0;
        }
        if SHOW_TIE_DETAIL == 1 {
          println!("扎紧 {} 部: {start} -> {t}", part.to_string());
        }
      }
      _ => {
        let p = self.part_process_mut(part);
        let start = *p;
        let end;
        *p += *rope;
        if *p >= 100 {
          let remain = *p - 100;
          *p = 100;
          end = 100;
          *self.part_is_mut(part) = true;
          *self.part_tightness_mut(part) = 100;
          *rope = remain;
        } else {
          end = *p;
          *rope = 0;
        }
        if SHOW_TIE_DETAIL == 1 {
          println!("捆绑 {} 部: {start} -> {end}", part.to_string());
        }
      }
    }
  }
  
  fn tie_main(&mut self, mut rope : i32) {
    for part in BoundPart::tie_order() {
      if rope > 0 {
        self.tie_part(part, &mut rope);
      }
    }
  }

  
    
}

fn state(is : bool, process : i32, tightness : i32) -> BoundState {
  if is {
    if tightness < 100 {
      BoundState::Loose
    } else {
      BoundState::Full
    }
  } else {
    if process > 0 {
      BoundState::Tieing
    } else {
      BoundState::None
    }
  }
}

impl Unit {
  pub fn bound_part_state(&self, part : BoundPart) -> BoundState {
    self.bound.part_state(part)
  }

  pub fn tie_ability(&self) -> i32 {
    self.skl() * 5 + 100
  }

  pub fn be_tie_exe(&mut self, rope : i32) {
    self.bound.tie_main(rope);
  }
}
