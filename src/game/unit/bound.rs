

#[derive(Debug, Clone, Default)]
pub struct Bound {
  wrist : bool,
  arm : bool,
  leg : bool,
  lock : bool,
}

impl Bound {
  pub fn new() -> Self {
    Self::default()
  }

  // 定性影响
  pub fn is_upper_able(&self) -> bool {
    !self.is_wrist()
  }

  pub fn is_lower_able(&self) -> bool {
    !self.is_leg()
  }

  // 定量影响
  pub fn move_coef(&self) -> f64 {
    let mut c = 1.0;
    if self.is_wrist() {c *= 0.8}
    if self.is_arm() {c *= 0.8}
    if self.is_leg() {c *= 0.5}
    if self.is_lock() {c *= 0.0}
    c
  }

  // 索引
  pub fn is_wrist(&self) -> bool {
    self.wrist
  }

  pub fn is_arm(&self) -> bool {
    self.arm
  }

  pub fn is_leg(&self) -> bool {
    self.leg
  }

  pub fn is_lock(&self) -> bool {
    self.lock
  }
    
}