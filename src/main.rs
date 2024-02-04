pub mod game;

use game::unit::Unit;
use game::board::Board;
use game::common::*;
use game::dice::*;

fn test_team() {
  let noel = Unit::new("诺艾尔", 10, 10, 10, Dir::Right);
  let elis = Unit::new("伊莉丝", 12, 18, 13, Dir::Right);
  let alyssa = Unit::new("艾丽莎", 11, 15, 16, Dir::Right);
  let yelin = Unit::new("叶  琳", 16, 14, 12, Dir::Right);
  let boss = Unit::new("奎  诺", 16, 20, 18, Dir::Left);
  let small1 = Unit::new("小弟甲", 12, 12, 12, Dir::Left);
  let small2 = Unit::new("小弟乙", 12, 12, 12, Dir::Left);
  let small3 = Unit::new("小弟丙", 12, 12, 12, Dir::Left);
  
  let mut board = Board::new( DiceWy::new(114514));
  board.insert_unit(noel, 0);
  board.insert_unit(elis, 0);
  board.insert_unit(alyssa, 0);
  board.insert_unit(yelin, 0);
  board.insert_unit(boss, 1);
  board.insert_unit(small1, 1);
  board.insert_unit(small2, 1);
  // board.insert_unit(small3, 1);

  board.play();
}

pub fn debug() {
  // let mut board = Board::new( DiceOne::new());
  let mut board = Board::new( DiceWy::new(114515));

  
  let noel = Unit::new("诺艾尔", 13, 13, 15, Dir::Right);
  let mut alyssa = Unit::new("艾丽莎", 11, 15, 16, Dir::Right);
  alyssa.fall_exe();
  alyssa.be_tie_exe(400);
  let boss = Unit::new("奎  诺", 12, 12, 12, Dir::Left);
  let small1 = Unit::new("小弟甲", 10, 10, 10, Dir::Left);

  // board.insert_unit(noel, 0);
  board.insert_unit(boss, 1);
  board.insert_unit(noel, 0);
  board.insert_unit(alyssa, 0);
  board.insert_unit(small1, 1);

  board.play()
}


fn main() {
  println!("Hello, world!");
  // test_team();
  debug();
  
}