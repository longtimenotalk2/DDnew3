pub mod game;

use game::unit::Unit;
use game::board::Board;
use game::common::*;

fn test_show() {
  let noel = Unit::new("诺艾尔", 10, 10, 10, Dir::Right);
  let boss = Unit::new("奎  诺", 16, 20, 18, Dir::Left);
  let board = Board::new(vec![(noel, 0), (boss, 1)]);
  board.show();
}

fn main() {
  println!("Hello, world!");
  test_show();
  
}