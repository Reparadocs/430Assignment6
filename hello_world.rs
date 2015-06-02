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
   println!("Hello World!");
   //let test_num = ExprC::numC {n : 3};
   let test_num = ExprC::binOpC {op : "+".to_string(), l : Box::new(ExprC::numC { n : 5 }), r : Box::new(ExprC::numC { n : 3}) };
   println!("{}", interp(test_num));
}
