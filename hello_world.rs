enum ExprC {
   numC { n : i32 },
   boolC { b : bool },
   idC { s : String },
   ifC { test : Box<ExprC>, then : Box<ExprC>, els : Box<ExprC> },
   appC { fun : Box<ExprC>, arg : Vec<ExprC> },
   binOpC { op : String, l : Box<ExprC>, r : Box<ExprC> },
   lamC { args : Vec<ExprC>, body : Box<ExprC> }
}

fn interp_binop(op: String, l: Box<ExprC>, r: Box<ExprC>) -> i32 {
   let left = interp(*l);
   let right = interp(*r);
   match op.as_ref() {
      "+" => left + right,
      "-" => left - right,
      "*" => left * right,
      "/" => left / right,
      _ => left - right
   }
}

fn interp(e: ExprC) -> i32 {
   match e {
      ExprC::numC { n: n } => n,
      ExprC::boolC { b: b} => 5,
      ExprC::binOpC { op: op, l: l, r: r } => interp_binop(op, l, r),
      _ => 10
   }
}

fn main() {
   let test_num1 = ExprC::binOpC {op : "+".to_string(), l : Box::new(ExprC::numC { n : 4 }), r : Box::new(ExprC::numC { n : 2}) };
   let test_num2 = ExprC::binOpC {op : "-".to_string(), l : Box::new(ExprC::numC { n : 4 }), r : Box::new(ExprC::numC { n : 2}) };
   let test_num3 = ExprC::binOpC {op : "*".to_string(), l : Box::new(ExprC::numC { n : 4 }), r : Box::new(ExprC::numC { n : 2}) };
   let test_num4 = ExprC::binOpC {op : "/".to_string(), l : Box::new(ExprC::numC { n : 4 }), r : Box::new(ExprC::numC { n : 2}) };
   println!("{} {} {} {}", interp(test_num1),
                           interp(test_num2),
                           interp(test_num3), 
                           interp(test_num4));
}

fn test() {
  assert!(true);
}
