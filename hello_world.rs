enum Value {
   numV { n : i32 },
   boolV { b : bool },
   closV { args : Vec<String>, body : Box<ExprC> }   
}

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
   match left {
      Value::numV { n: l_n } =>
         match right {
            Value::numV { n: r_n} =>
               match op.as_ref() {
                  "+" => l_n + r_n,
                  "-" => l_n - r_n,
                  "*" => l_n * r_n,
                  "/" => l_n / r_n,
                  _ => panic!("Not binop"),
               },
            _ => panic!("Bad"),
         },
      _ => panic!("Bad"),
   }
}

fn interp(e: ExprC) -> Value {
   match e {
      ExprC::numC { n: n } => Value::numV {n : n},
      ExprC::boolC { b: b} => Value::boolV {b : b},
      ExprC::binOpC { op: op, l: l, r: r } => Value::numV {n : interp_binop(op, l, r)},
      _ => panic!("Not implemented"),
   }
}

fn serialize_bool(b: bool) {
   if b {
      println!("True");
   } else {
      println!("False");
   }
}

fn serialize(v: Value) {
   match v {
      Value::numV { n: n } => println!("{}", n),
      Value::boolV { b: b } => serialize_bool(b),
      Value::closV { args: args, body: body } => println!("#<procedure>"),
   }
}

fn main() {
   let test_num1 = ExprC::binOpC {op : "+".to_string(), l : Box::new(ExprC::numC { n : 4 }), r : Box::new(ExprC::numC { n : 2}) };
   let test_num2 = ExprC::binOpC {op : "-".to_string(), l : Box::new(ExprC::numC { n : 4 }), r : Box::new(ExprC::numC { n : 2}) };
   let test_num3 = ExprC::binOpC {op : "*".to_string(), l : Box::new(ExprC::numC { n : 4 }), r : Box::new(ExprC::numC { n : 2}) };
   let test_num4 = ExprC::binOpC {op : "/".to_string(), l : Box::new(ExprC::numC { n : 4 }), r : Box::new(ExprC::numC { n : 2}) };
   println!("{} {} {} {}", serialize(interp(test_num1)),
                           serialize(interp(test_num2)),
                           serialize(interp(test_num3)), 
                           serialize(interp(test_num4)));
}

fn test() {
  assert!(true);
}
