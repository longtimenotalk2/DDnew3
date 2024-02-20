// 全局开关
pub const SHOW_BATTLE_EXPECT : u32 = 1;
pub const SHOW_BATTLE_DETAIL : u32 = 1;
pub const SHOW_TIE_DETAIL : u32 = 1;

// 类型
pub type Id = u32;
pub type Team = u32;
pub type Pos = i32;

// 结构
// acc - evd = hit
// pir - asd = stt
// whk - rfg = cri

#[derive(Debug, Clone)]
pub struct AttackInput {
  pub atk : i32,
  pub acc : i32,
  pub pir : i32,
  pub whk : i32,
}

#[derive(Debug, Clone)]
pub struct AttackAnalyse {
  pub hit : i32,
  pub stt : i32,
  pub cri : i32,
  pub dmg_asd : i32,
  pub dmg_stt : i32,
  pub dmg_cri : i32,
}

impl AttackAnalyse {
  pub fn to_string(&self) -> String {
    format!("命{}, 穿{}, 爆{}, {}/{}/{}", self.hit, self.stt, self.cri, self.dmg_asd, self.dmg_stt, self.dmg_cri)
  }

  pub fn expect_damage(&self) -> f64 {
    let mut r = 0.;
    let hit = self.hit as f64 / 100.;
    let stt = self.stt as f64 / 100.;
    let cri = self.cri as f64 / 100.;
    let dmg_asd = self.dmg_asd as f64;
    let dmg_stt = self.dmg_stt as f64;
    let dmg_cri = self.dmg_cri as f64;
    r += hit * stt * cri * dmg_cri;
    r += hit * stt * (1. - cri) * dmg_stt;
    r += hit * (1. - stt) * dmg_asd;
    r
  }
}

#[derive(Debug, Clone)]
pub struct AttackResult {
  dmg : i32,
  is_hit : bool,
  is_asd : bool,
  is_stt : bool,
  is_cri : bool,
  analyse : Option<AttackAnalyse>,
}

impl AttackResult {
  pub fn from_analyse(ana : &AttackAnalyse, r_hit : i32, r_stt : i32, r_cri : i32) -> Self {
    if r_hit > ana.hit {
      // 未命中
      Self::new(0, false, false, ana)
    } else {
      if r_stt > ana.stt {
        // 格挡住了
        Self::new(ana.dmg_asd, true, false, ana)
      } else {
        if r_cri > ana.cri {
          // 直击未暴击
          Self::new(ana.dmg_stt, false, false, ana)
        } else {
          // 暴击
          Self::new(ana.dmg_cri, false, true, ana)
        }
      }
    }
  }
  
  pub fn new(dmg : i32, is_asd : bool, is_cri : bool, ana : &AttackAnalyse) -> Self {
    let is_hit = dmg > 0;
    let is_stt = is_hit && !is_asd;
    Self {
      dmg,
      is_hit,
      is_asd,
      is_stt,
      is_cri,
      analyse : Some(ana.clone()),
    }
  }

  pub fn dmg(&self) -> i32 {
    self.dmg
  }
  pub fn is_hit(&self) -> bool {
    self.is_hit
  }
  pub fn is_asd(&self) -> bool {
    self.is_asd
  }
  pub fn is_stt(&self) -> bool {
    self.is_stt
  }
  pub fn is_cri(&self) -> bool {
    self.is_cri
  }

  pub fn analyse(&self) -> Option<&AttackAnalyse> {
    self.analyse.as_ref()
  }
}

#[derive(Debug, Clone)]
pub struct Target {
  pos : Option<Pos>,
  dir : Option<Dir>,
}

impl Target {
  pub fn new_move(pos : Pos, dir : Dir) -> Self {
    Self {
      pos: Some(pos),
      dir: Some(dir),
    }
  }

  pub fn new_attack(pos : Pos, dir : Dir) -> Self {
    Self {
      pos: Some(pos),
      dir: Some(dir),
    }
  }

  pub fn empty() -> Self {
    Self {
      pos: None,
      dir: None,
    }
  }

  pub fn to_string(&self) -> String {
    format!("{}位置{}", self.dir.unwrap().to_string(), pos2string(self.pos.unwrap()))
  }

  pub fn to_string_anti(&self) -> String {
    format!("{}位置{}", self.dir.unwrap().anti().to_string(), pos2string(self.pos.unwrap()))
  }

  pub fn pos(&self) -> Option<Pos> {
    self.pos
  }

  pub fn dir(&self) -> Option<Dir> {
    self.dir
  }
}



// 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
  Left,
  Right,
}

impl Dir {
  pub fn to_string(&self) -> String {
    match self {
      Dir::Left => "↑".to_string(),
      Dir::Right => "↓".to_string(),
    }
  }

  pub fn anti(self) -> Self {
    match self {
      Dir::Left => Dir::Right,
      Dir::Right => Dir::Left,
    }
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Ai {
  Basic,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Skill {
  Punch,
  Kick,
  Tie,
  Untie,
  Move,
  MoveTurn,
  Pass,
}

impl Skill {
  pub fn iter_sense() -> impl Iterator<Item = Self> {
    [
      Self::Punch,
      Self::Kick,
      Self::Tie,
      Self::Untie,
      Self::Move, 
      Self::MoveTurn,
    ].iter().cloned()
  }

  pub fn sort(v : &mut Vec<Self>) {
    let mut v2 = Vec::new();
    for skl in Self::iter_sense() {
      if v.contains(&skl){
        v2.push(skl);
      }
    }
    *v = v2;
  }

  pub fn to_string(&self) -> String {
    match self {
      Self::Punch => "挥拳",
      Self::Kick => "踢腿",
      Self::Tie => "捆绑",
      Self::Untie => "解绑",
      Self::Move => "移动",
      Self::MoveTurn => "移动并转身",
      Self::Pass => "略过",
    }.to_string()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundState {
  None,
  Full,
  Tieing,
  Loose,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BoundPart {
  Wrist,
  Arm,
  Leg,
  Lock,
}

impl BoundPart {
  pub fn tie_order() -> Vec<Self> {
    vec!(Self::Wrist, Self::Leg, Self::Arm, Self::Lock)
  }

  pub fn struggle_order() -> Vec<Self> {
    vec!(Self::Lock, Self::Leg, Self::Arm, Self::Wrist)
  }

  pub fn untie_order() -> Vec<Self> {
    vec!(Self::Lock, Self::Arm, Self::Wrist, Self::Leg)
  }

  pub fn to_string(&self) -> String {
    match self {
      Self::Wrist => "腕".to_string(),
      Self::Arm => "臂".to_string(),
      Self::Leg => "腿".to_string(),
      Self::Lock => "锁".to_string(),
    }
  }
}

// 函数
pub fn i2lv(i : i32) -> Option<i32> {
  if i <= 0 {
    None
  } else {
    Some(i/5)
  }
}

pub fn i2pro(i : i32) -> i32 {
  (i.max(0)).min(100)
}

pub fn i2dmg(i : i32) -> i32 {
  i.max(1)
}

pub fn pos2string(pos : Pos) -> String {
  let l = "ABCDEFGHIJKLMN";
  // 取出l中第pos个字符
  l.chars().nth(pos as usize).unwrap().to_string()
  
}

// 接口
pub trait Dice {
  fn d100(&mut self) -> i32;
}
