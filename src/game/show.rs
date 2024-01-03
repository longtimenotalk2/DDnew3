// 专门管理显示的模块

const RED : &str = "\u{1b}[31m";
const GREEN : &str = "\u{1b}[32m";
const YELLOW : &str = "\u{1b}[33m";
const BLUE : &str = "\u{1b}[34m";
const RESET : &str = "\u{1b}[m";

use super::board::pawn::Pawn;
use super::board::Board;
use crate::game::common::*;

#[derive(Debug, Clone)]
enum BoundUpper {
  None,
  Wrist,
  Full,
}

#[derive(Debug, Clone)]
enum BoundLower {
  None,
  Leg,
  Lock,
}

struct PawnShowList {
  name : String,
  team : Team,
  dir : Option<Dir>,
  is_stand : bool,
  is_pin : bool,
  is_tieing : bool,
  is_ctrled : bool,
  is_stun : bool,
  stun_turn : i32,
  bound_upper : BoundUpper,
  bound_lower : BoundLower,
  str : i32,
  skl : i32,
  spd : i32,
}

impl PawnShowList {
  pub fn show_one_line(&self) {
    // 艾丝妲↓捆     (力 5 技12 速16) 
    // 玲  可 晕 臂锁 (力 2 技 2 速 2) 

    // 站立时，要么捆要么牵，倒地时，要么倒要么晕（带一个数字，写在箭头位置）
    
    let mut line = String::new();
    
    // 名字
    let color = match self.team {
      0 => BLUE,
      1 => RED,
      _ => unreachable!(),
    };
    let name = format!("{}{}{}", color, self.name, RESET);
    line += &name;

    // 朝向（以及眩晕回合）
    let dir = match self.dir {
      None => if self.stun_turn > 0 {
        self.stun_turn.to_string()
      } else {" ".to_string()},
      Some(Dir::Left) => "↑".to_string(),
      Some(Dir::Right) => "↓".to_string(),
    };
    line += &dir;

    // 状态（一个汉字）
    let state = if self.is_stun {
      "晕"
    } else if self.is_stand {
      if self.is_tieing {
        "捆"
      } else if self.is_pin {
        "牵"
      } else {
        "  "
      }
    } else {
      if self.is_ctrled {
        "控"
      }else {
        "倒"
      }
    };
    line += &state;

    // 绳索
    line += " ";
    line += match self.bound_upper {
      BoundUpper::None => "  ",
      BoundUpper::Wrist => "腕",
      BoundUpper::Full => "臂",
    };
    line += match self.bound_lower {
      BoundLower::None => "  ",
      BoundLower::Leg => "腿",
      BoundLower::Lock => "锁",
    };
    line += " ";

    // 力、技、速
    line += &format!("(力{:2} 技{:2} 速{:2})", self.str, self.skl, self.spd);
    
    
    // 输出
    println!("{}", line);
  }
}

impl Board {
  pub fn show_one_line(&self) {
    for pawn in self.pawns() {
      pawn.show_list().show_one_line();
    }
  }
}

impl Pawn {
  fn show_list(&self) -> PawnShowList {
    let unit = self.unit();
    let name = &unit.name;
    let team = self.team();
    let dir = unit.pose().dir();
    let is_stand = unit.pose().is_stand();
    let is_pin = unit.pose().is_pin();
    let is_tieing = unit.pose().is_tieing();
    let is_ctrled = unit.pose().is_ctrled();
    let is_stun = unit.state().is_stun();
    let stun_turn = unit.state().stun_turn();
    let bound_upper = if unit.bound().is_arm() {
      BoundUpper::Full
    } else if unit.bound().is_wrist() {
      BoundUpper::Wrist
    } else {
      BoundUpper::None
    };
    let bound_lower = if unit.bound().is_lock() {
      BoundLower::Lock
    } else if unit.bound().is_leg() {
      BoundLower::Leg
    } else {
      BoundLower::None
    };
    let str = unit.str();
    let skl = unit.skl();
    let spd = unit.spd();
    
    
    PawnShowList {
      name : name.to_string(),
      team,
      dir,
      is_stand,
      is_pin,
      is_tieing,
      is_ctrled,
      is_stun,
      stun_turn,
      bound_upper,
      bound_lower,
      str,
      skl,
      spd,
    }
  } 
}