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

  let ai0 = true;
  let ai1 = true;
  let dice = DiceWy::new(114516);
  let round_limit = Some(10);
  let mut board = Board::new(dice , ai0, ai1, round_limit);
  
  board.insert_unit(noel, 0);
  board.insert_unit(elis, 0);
  board.insert_unit(alyssa, 0);
  board.insert_unit(yelin, 0);
  board.insert_unit(boss, 1);
  board.insert_unit(small1, 1);
  board.insert_unit(small2, 1);
  board.insert_unit(small3, 1);

  
  let r = board.play();
  println!("{}", r.to_string());
}




fn main() {
  println!("Hello, world!");
  test_team();
  //debug();
  
}