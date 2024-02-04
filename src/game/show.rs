// 专门管理显示的模块

const RED : &str = "\u{1b}[31m";
const GREEN : &str = "\u{1b}[32m";
const YELLOW : &str = "\u{1b}[33m";
const BLUE : &str = "\u{1b}[34m";
const RESET : &str = "\u{1b}[m";

use super::board::pawn::Pawn;
use super::board::Board;
use crate::game::common::*;

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
  wrist : BoundState,
  arm : BoundState,
  leg : BoundState,
  lock : BoundState,
  str : i32,
  skl : i32,
  spd : i32,
  hurt : i32,
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
        "扰"
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
    let mut arm = "臂".to_string();
    match self.arm {
      BoundState::None => arm = "  ".to_string(),
      BoundState::Full => (),
      BoundState::Tieing => arm = add_color(&arm, RED),
      BoundState::Loose => arm = add_color(&arm, YELLOW),
    }
    let mut wrist = "腕".to_string();
    match self.wrist {
      BoundState::None => wrist = "  ".to_string(),
      BoundState::Full => (),
      BoundState::Tieing => wrist = add_color(&wrist, RED),
      BoundState::Loose => wrist = add_color(&wrist, YELLOW),
    }
    let mut leg = "腿".to_string();
    match self.leg {
      BoundState::None => leg = "  ".to_string(),
      BoundState::Full => (),
      BoundState::Tieing => leg = add_color(&leg, RED),
      BoundState::Loose => leg = add_color(&leg, YELLOW),
    }
    let mut lock = "锁".to_string();
    match self.lock {
      BoundState::None => lock = "  ".to_string(),
      BoundState::Full => (),
      BoundState::Tieing => lock = add_color(&lock, RED),
      BoundState::Loose => lock = add_color(&lock, YELLOW),
    }
    line += &format!(" {arm}{wrist}{leg}{lock}");

    // 力、技、速
    line += &format!("(力{:2}技{:2}速{:2})", self.str, self.skl, self.spd);

    // 受伤
    line += &format!("伤{:2}", self.hurt);
    
    
    // 输出
    println!("{}", line);
  }
}

impl Board {
  pub fn show_one_line(&self, ids : &[Id]) {
    for (i, pawn) in self.pawns().iter().enumerate() {
      // 状态码
      let id = pawn.id();
      let m = if ids.contains(&id) {
        ">"
      } else if pawn.unit().is_wait() {
        "w"
      } else if pawn.unit().is_action() {
        if pawn.unit().can_action_sense() {
          "|"
        } else {
          "x"
        }
      } else {" "};
      // 位置
      let p = pos2string(i as Pos);
      print!("{p}{m} ");
        pawn.show_list().show_one_line();
    }
  }
}

impl Pawn {
  fn show_list(&self) -> PawnShowList {
    let unit = self.unit();
    let name = &unit.name;
    let team = self.team();
    let dir = unit.dir();
    let is_stand = unit.is_stand();
    let is_pin = unit.is_pin();
    let is_tieing = unit.is_tieing();
    let is_ctrled = unit.is_ctrled();
    let is_stun = unit.is_stun();
    let stun_turn = unit.stun_turn();
    let arm = unit.bound_part_state(BoundPart::Arm);
    let wrist = unit.bound_part_state(BoundPart::Wrist);
    let leg = unit.bound_part_state(BoundPart::Leg);
    let lock = unit.bound_part_state(BoundPart::Lock);
    let str = unit.str();
    let skl = unit.skl();
    let spd = unit.spd();
    let hurt = unit.hurt();
    
    
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
      arm,
      wrist,
      leg,
      lock,
      str,
      skl,
      spd,
      hurt,
    }
  } 
}

impl AttackResult {
  pub fn show(&self) {
    if let Some(ana) = self.analyse() {
      let mut dmg_asd = format!("{}", ana.dmg_asd);
      let mut dmg_stt = format!("{}", ana.dmg_stt);
      let mut dmg_cri = format!("{}", ana.dmg_cri);
      let hit = if self.is_hit() {

        add_color(&format!("{}", ana.hit), GREEN)
      } else {
        add_color(&format!("{}", ana.hit), RED)
        
      };
      let stt = if self.is_stt() {
        add_color(&format!("{}", ana.stt), GREEN)
      } else {
        add_color(&format!("{}", ana.stt), RED)
      };
      let cri = if self.is_cri() {
        add_color(&format!("{}", ana.cri), GREEN)
      } else {
        add_color(&format!("{}", ana.cri), RED)
      };
      if self.is_cri() {
        dmg_cri = add_color(&dmg_cri, GREEN);
      } else if self.is_stt() {
        dmg_stt = add_color(&dmg_stt, GREEN);
      } else if self.is_hit() {
        dmg_asd = add_color(&dmg_asd, GREEN);
      }

      let mut r = String::new();
      if self.dmg() > 0 {
        r += &format!("{}", self.dmg());
        if self.is_cri() {
          r += "!";
        }
      } else {
        r += "落空";
      }
      
      println!("命{hit}, 穿{stt}, 爆{cri} ({dmg_asd}/{dmg_stt}/{dmg_cri}) {r}")
    }
  }
}

fn add_color(s : &str, color : &str) -> String {
  format!("{}{}{}", color, s, RESET)
}