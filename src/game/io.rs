use std::io;

pub fn io(title: String, options : &[String], default : Option<usize>) -> usize {
  println!("{}", title);
  // 显示所有选项
  for (i, option) in options.iter().enumerate() {
    println!("{}: {}", i, option);
  }
  // 接受用户输入，如果正确则返回对应的索引
  let len = options.len();
  loop {
    let mut ops = String::new();
    io::stdin().read_line(&mut ops).expect("failed to read line");
    if default.is_some() && &ops == "\n" {
      return default.unwrap()
    }
    if let Ok(op) = ops.trim().parse::<usize>() {
      if op < len {
        return op
      } else {
        println!("输入错误,请输入所给选项前面的数字");
      }
    }else {
      println!("输入错误,请输入一个自然数");
    }
  }
}

// pub fn io_unit(&self, ids : &[u32], can_wait : bool) -> Option<u32> {
//   if ids.len() == 1 && !can_wait {
//     return Some(ids[0])
//   }
//   println!("请选择希望行动的角色：") ;
//   for (_, id) in ids.iter().enumerate() {
//     let u = &self.pos_pawn(self.id_pos(*id)).unwrap().unit;
//     print!("{} : {}", id, u.name);
//     // 行动提示：控制目标，攻击穿透目标
//     println!("({})", self.skill_hint(*id));
//   }

//   if can_wait {
//     print!("0 : 等待");
//     // 行动提示
//     println!("({})", self.enemy_skill_hint());
//   }

  
//   loop {
//     let mut ops = String::new();
//     io::stdin().read_line(&mut ops).expect("failed to read line");
//     if &ops == "\n" {
//       return Some(self.ai_unit(ids))
//     }
//     if let Ok(op) = ops.trim().parse::<u32>() {
//       if ids.contains(&op) {
//         return Some(op);
//       } else if can_wait && op == 0 {
//         return None;
//       } else {
//         println!("输入错误,请输入所给选项前面的数字");
//       }
//     }else {
//       println!("输入错误,请输入一个自然数");
//     }
//   }
// }