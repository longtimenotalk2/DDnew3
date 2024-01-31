use crate::game::common::*;
use crate::game::unit::Unit;

#[derive(Debug, Clone)]
pub struct Pose {
  dir : Option<Dir>, // 仅站立时才有方向
  stand : bool,
  pin : bool, // 仅站立时才有牵制
  tieing : bool, // 仅站立时，正在捆绑别人
  ctrled : bool, // 仅倒地时，被人控制中
  tieing_id : Option<Id>,
  ctrled_id : Option<Id>,
}

impl Pose {
  pub fn new(dir:Dir) -> Self {
    Self {
      dir : Some(dir),
      stand: true,
      pin: false,
      tieing : false,
      ctrled : false,
      tieing_id : None,
      ctrled_id : None,
    }
  }

  // 定性状态

  pub fn is_backtrab(&self, dir : Dir) -> Option<bool> {
    match self.dir {
      Some(d) => {
        if d == dir {
          Some(true)
        } else {
          Some(false)
        }
      },
      None => {
        None
      },
    }
  }

  pub fn is_pin(&self) -> bool {
    self.pin
  }

  pub fn is_stand(&self) -> bool {
    self.stand
  }

  pub fn dir(&self) -> Option<Dir> {
    self.dir
  }

  pub fn is_tieing(&self) -> bool {
    self.tieing
  }

  pub fn is_ctrled(&self) -> bool {
    self.ctrled
  }


  // 定量影响
  pub fn react_coef(&self, dir : Dir) -> f64 {
    let mut c = 1.0;
    if self.stand {
      if self.is_backtrab(dir) == Some(true) {
        c *= 0.7;
        if self.is_pin() {
          c *= 0.7;
        }
      }
    } 
    c
  }

  pub fn move_coef(&self) -> f64 {
    let mut c = 1.0;
    if !self.stand {
      c *= 0.5;
    }
    c
  }
  
  // 变动

  pub fn fall_exe(&mut self) {
    self.stand = false;
    self.dir = None;
  }
  
  pub fn pin_exe(&mut self) {
    self.pin = true;
  }

  pub fn pin_cancel(&mut self) {
    self.pin = false;
  }
}

impl Unit {
  pub fn dir(&self) -> Option<Dir> {
    self.pose.dir()
  }

  pub fn is_stand(&self) -> bool {
    self.pose.is_stand()
  }

  pub fn is_pin(&self) -> bool {
    self.pose.is_pin()
  }

  pub fn is_tieing(&self) -> bool {
    self.pose.is_tieing()
  }

  pub fn is_ctrled(&self) -> bool {
    self.pose.is_ctrled()
  }

  pub fn tieing_id(&self) -> Option<Id> {
    self.pose.tieing_id
  }

  pub fn ctrled_id(&self) -> Option<Id> {
    self.pose.ctrled_id
  }
  
  // 变动
  pub fn set_dir(&mut self, dir : Dir) {
    self.pose.dir = Some(dir);
  }

  pub fn start_tieing_to(&mut self, id : Id) {
    self.pose.tieing = true;
    self.pose.tieing_id = Some(id);
  }

  pub fn start_be_tied(&mut self, id : Id) {
    self.pose.ctrled = true;
    self.pose.ctrled_id = Some(id);
  }
  
  pub fn cancel_tieing(&mut self) -> Id {
    let id = self.pose.tieing_id.take().unwrap();
    self.pose.tieing = false;
    id
  }
  pub fn cancel_be_tied(&mut self) -> Id {
    let id = self.pose.ctrled_id.take().unwrap();
    self.pose.ctrled = false;
    id
  }

}