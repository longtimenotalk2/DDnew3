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

// 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dir {
  Left,
  Right,
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
