// 类型
pub type Id = u32;
pub type Team = u32;
pub type Pos = i32;

// 结构
// acc - evd = hit
// pir - asd = dir
// whk - rfg = cri

#[derive(Debug, Clone)]
pub struct AttackInput {
  pub atk : i32,
  pub acc : i32,
  pub pir : i32,
  pub whk : i32,
}

#[derive(Debug, Clone)]
pub struct AttactAnalyse {
  pub hit : i32,
  pub stt : i32,
  pub cri : i32,
  pub dmg_asd : i32,
  pub dmg_stt : i32,
  pub dmg_cri : i32,
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

  pub fn empty() -> Self {
    Self {
      pos: None,
      dir: None,
    }
  }

  pub fn to_string(&self, skl : Skill) -> String {
    match skl {
      Skill::Move => self.to_string_move(),
      Skill::Pass => "无目标".to_string(),
    }
  }

  fn to_string_move(&self) -> String {
    format!("{}位置{}", self.dir.unwrap().to_string(), self.pos.unwrap())
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Skill {
  Move,
  Pass,
}

impl Skill {
  pub fn iter() -> impl Iterator<Item = Self> {
    [Self::Move, Self::Pass].iter().cloned()
  }

  pub fn sort(v : &mut Vec<Self>) {
    let mut v2 = Vec::new();
    for skl in Self::iter() {
      if v.contains(&skl){
        v2.push(skl);
      }
    }
    *v = v2;
  }

  pub fn to_string(&self) -> String {
    match self {
      Self::Move => "移动",
      Self::Pass => "略过",
    }.to_string()
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
