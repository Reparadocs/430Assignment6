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

fn interp_ifC(test: Box<ExprC>, this: Box<ExprC>, els: Box<ExprC>) -> Value {
   let test_val = interp(*test);
   let this_val = interp(*this);
   let els_val = interp(*els);
   match test_val {
      Value::boolV {b: test_b} =>
         match test_b {
            false => this_val,
            true => els_val,
         },
      _ => panic! ("Not a bool"),
   }
}

fn interp_binop(op: String, l: Box<ExprC>, r: Box<ExprC>) -> Value {
   let left = interp(*l);
   let right = interp(*r);
   match left {
      Value::numV { n: l_n } =>
         match right {
            Value::numV { n: r_n} =>
               match op.as_ref() {
                  "+" => Value::numV {n : l_n + r_n},
                  "-" => Value::numV {n : l_n - r_n},
                  "*" => Value::numV {n : l_n * r_n},
                  "/" => Value::numV {n : l_n / r_n},
                  "<=" => Value::boolV {b : l_n <= r_n},
                  "eq?" => Value::boolV {b : l_n == r_n},
                  _ => panic!("Not binop"),
               },
            _ =>
               match op.as_ref() {
                  "eq?" => Value::boolV {b : false},
                  _ => panic!("Type mismatch!"),
               },
         },
      Value::boolV { b : l_b } =>
         match right {
            Value::boolV { b : r_b} =>
               match op.as_ref() {
                  "eq?" => Value::boolV {b : l_b == r_b},
                  _ => panic!("Type mismatch!"), 
               },
            _ => 
            match op.as_ref() {
                  "eq?" => Value::boolV {b : false},
                  _ => panic!("Type mismatch!"),
               },
         },
      _ =>
         match op.as_ref() {
                  "eq?" => Value::boolV {b : false},
                  _ => panic!("Type mismatch!"),
               },
   }
}

fn interp(e: ExprC) -> Value {
   match e {
      ExprC::numC { n: n } => Value::numV {n : n},
      ExprC::boolC { b: b} => Value::boolV {b : b},
      ExprC::lamC {args: args, body: body} => Value::closV {args : args, body : body},
      ExprC::binOpC { op: op, l: l, r: r } => interp_binop(op, l, r),

      _ => panic!("Not implemented"),
   }
}

fn serialize_bool(b: bool) -> String {
   if b {
      "True".to_string()
   } else {
      "False".to_string()
   }
}

fn serialize(v: Value) -> String {
   match v {
      Value::numV { n: n } => format!("{}", n),
      Value::boolV { b: b } => serialize_bool(b),
      Value::closV { args: args, body: body } => "#<procedure>".to_string(),
   }
}

fn top_eval(e : ExprC) -> String {
   serialize(interp(e))
}

fn main() {
   println!("Hello World!");
   
   //Primitive Tests
   assert_eq!(top_eval(ExprC::numC {n : 3}), "3");
   assert_eq!(top_eval(ExprC::boolC {b : false}), "False");
   assert_eq!(top_eval(ExprC::lamC {args : vec!["a".to_string(), "b".to_string(), "c".to_string()], body: Box::new(ExprC::numC { n : 3})}), "#<procedure>");

   //Binop Tests
   assert_eq!(top_eval(ExprC::binOpC {op : "eq?".to_string(), l : Box::new(ExprC::boolC { b : false}), r : Box::new(ExprC::boolC {b : true})}), "False");
   assert_eq!(top_eval(ExprC::binOpC {op : "eq?".to_string(), l : Box::new(ExprC::boolC { b : false}), r : Box::new(ExprC::boolC {b : false})}), "True");
   assert_eq!(top_eval(ExprC::binOpC {op : "eq?".to_string(), l : Box::new(ExprC::boolC { b : true}), r : Box::new(ExprC::numC {n : 3})}), "False");
   assert_eq!(top_eval(ExprC::binOpC {op : "eq?".to_string(), l : Box::new(ExprC::numC { n : 3}), r : Box::new(ExprC::numC {n : 3})}), "True");
   assert_eq!(top_eval(ExprC::binOpC {op : "eq?".to_string(), l : Box::new(ExprC::numC { n : 5}), r : Box::new(ExprC::numC {n : 3})}), "False");
   assert_eq!(top_eval(ExprC::binOpC {op : "<=".to_string(), l : Box::new(ExprC::numC { n : 3}), r : Box::new(ExprC::numC {n : 6})}), "True");
   assert_eq!(top_eval(ExprC::binOpC {op : "<=".to_string(), l : Box::new(ExprC::numC { n : 10}), r : Box::new(ExprC::numC {n : 3})}), "False");
   assert_eq!(top_eval(ExprC::binOpC {op : "<=".to_string(), l : Box::new(ExprC::numC { n : 5}), r : Box::new(ExprC::numC {n : 5})}), "True");
   assert_eq!(top_eval(ExprC::binOpC {op : "/".to_string(), l : Box::new(ExprC::numC { n : 15}), r : Box::new(ExprC::numC {n : 3})}), "5");
   assert_eq!(top_eval(ExprC::binOpC {op : "*".to_string(), l : Box::new(ExprC::numC { n : 5}), r : Box::new(ExprC::numC {n : 3})}), "15");
   assert_eq!(top_eval(ExprC::binOpC {op : "-".to_string(), l : Box::new(ExprC::numC { n : 10}), r : Box::new(ExprC::numC {n : 6})}), "4");
   assert_eq!(top_eval(ExprC::binOpC {op : "+".to_string(), l : Box::new(ExprC::numC { n : 5}), r : Box::new(ExprC::numC {n : 3})}), "8");
}
