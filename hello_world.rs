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
   lamC { args : Vec<String>, body : Box<ExprC> }
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
      ExprC::lamC {args: args, body: body} => Value::closV {args : args, body : body},
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
   println!("Hello World!");
   //let test_num = ExprC::numC {n : 3};
   let test_num = ExprC::binOpC {op : "+".to_string(), l : Box::new(ExprC::numC { n : 5 }), r : Box::new(ExprC::numC { n : 3}) };
   serialize(interp(test_num));
}
