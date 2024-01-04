pub mod game;

use game::unit::Unit;
use game::board::Board;
use game::common::*;

fn test_show() {
  let noel = Unit::new("诺艾尔", 10, 10, 10, Dir::Right);
  let elis = Unit::new("伊莉丝", 12, 18, 13, Dir::Right);
  let alyssa = Unit::new("艾丽莎", 11, 15, 16, Dir::Right);
  let yelin = Unit::new("叶  琳", 16, 14, 12, Dir::Right);
  let boss = Unit::new("奎  诺", 16, 20, 18, Dir::Left);
  let small1 = Unit::new("小弟甲", 12, 12, 12, Dir::Left);
  let small2 = Unit::new("小弟乙", 12, 12, 12, Dir::Left);
  let small3 = Unit::new("小弟丙", 12, 12, 12, Dir::Left);
  let board = Board::new(vec![(noel, 0),(elis, 0), (alyssa, 0), (yelin, 0),  (boss, 1), (small1, 1), (small2, 1),(small3, 1)]);
  board.show();

  dbg!(board.move_option(6));
}

fn main() {
  println!("Hello, world!");
  test_show();
  
}