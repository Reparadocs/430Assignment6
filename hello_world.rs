#![feature(convert, collections)]

enum Value {
   numV { n : i32 },
   boolV { b : bool },
   closV { args : Vec<String>, body : Box<ExprC>, cenv : Env }   
}

impl Clone for Value {
  fn clone(&self) -> Value {
    match *self {
      Value::numV {n : n} => Value::numV{n : n},
      Value::boolV {b : b} => Value::boolV{b : b},
      Value::closV {args : ref args, body: ref body, cenv: ref cenv} =>
        Value::closV{args: args.clone(), body: Box::new((*(*body)).clone()) , cenv: cenv.clone()}
    }
  }
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

impl Clone for ExprC {
  fn clone(&self) -> ExprC {
    match *self {
      ExprC::numC {n : n} => ExprC::numC {n : n},
      ExprC::boolC {b : b} => ExprC::boolC {b : b},
      ExprC::idC {s : ref s} => ExprC::idC {s : s.clone()},
      ExprC::ifC {test : ref test, then : ref then, els : ref els} => 
        ExprC::ifC {test : Box::new((*(*test)).clone()), 
        then : Box::new((*(*then)).clone()), els : Box::new((*(*els)).clone())},
      ExprC::appC {fun : ref fun, arg : ref arg} => 
        ExprC::appC {fun : Box::new((*(*fun)).clone()), arg : arg.clone()},
      ExprC::binOpC { op : ref op, l : ref l, r : ref r} =>
        ExprC::binOpC {op : op.clone(), l : Box::new((*(*l)).clone()), 
        r : Box::new((*(*r)).clone())},
      ExprC::lamC {args : ref args, body : ref body} =>
        ExprC::lamC {args : args.clone(), body: Box::new((*(*body)).clone())},
    }
  }
}

struct Binding {
  name: String,
  val: Value,
}

impl Clone for Binding {
  fn clone(&self) -> Binding {
    Binding { name: self.name.clone(), val: self.val.clone() }
  }
}

type Env = Vec<Binding>;

fn interp_ifC(test: Box<ExprC>, this: Box<ExprC>, els: Box<ExprC>, env: &[Binding]) -> Value {
   let test_val = interp(*test, env);
   let this_val = interp(*this, env);
   let els_val = interp(*els, env);
   match test_val {
      Value::boolV {b: test_b} =>
         match test_b {
            false => els_val,
            true => this_val,
         },
      _ => panic! ("Not a bool"),
   }
}

fn interp_binop(op: String, l: Box<ExprC>, r: Box<ExprC>, env: &[Binding]) -> Value {
   let left = interp(*l, env);
   let right = interp(*r, env);
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

fn interp_app(fun: Box<ExprC>, arg: Vec<ExprC>, env: &[Binding]) -> Value {
  let fd = interp(*fun, env);

  match fd {
    Value::closV { args: c_args, body: body, cenv: c_env } => interp(*body, add_params(c_args, arg, c_env.as_slice(), env).as_slice()),
    _ => panic!("Not a closure"),
  }
}

fn interp(e: ExprC, env: &[Binding]) -> Value {
   match e {
      ExprC::idC { s: s } => env_lookup(s, env),
      ExprC::numC { n: n } => Value::numV {n : n},
      ExprC::boolC { b: b} => Value::boolV {b : b},
      ExprC::lamC {args: args, body: body} => create_closure(args, body, env),
      ExprC::appC {fun: fun, arg: arg} => interp_app(fun, arg, env),
      ExprC::binOpC { op: op, l: l, r: r } => interp_binop(op, l, r, env),
      ExprC::ifC { test, then, els } => interp_ifC(test, then, els, env),

   }
}

fn create_closure(args : Vec<String>, body: Box<ExprC>, env: &[Binding]) -> Value {
  let mut ret: Vec<Binding> = Vec::new();
  ret.clone_from_slice(env);
  Value::closV {args : args, body : body, cenv : ret}
}

fn env_lookup(s: String, env: &[Binding]) -> Value {
  for b in env.iter() {
    if b.name == s {
      return b.val.clone()
    }
  }
  panic!("No match found in env")
}

fn add_params(params: Vec<String>, vals: Vec<ExprC>, ontopof: &[Binding], env: &[Binding]) -> Env {
  let mut oto: Vec<Binding> = Vec::new();
  for i in 0..ontopof.len() {
    oto.push(ontopof[i].clone());
  }

  for i in 0..params.len() {
    oto.push(Binding { name : params[i].clone(), val : interp(vals[i].clone(), env)});
  }
  oto
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
      Value::closV { args: args, body: body, cenv: cenv } => "#<procedure>".to_string(),
   }
}

fn top_eval(e : ExprC) -> String {
   let empty_vec: Vec<Binding> = Vec::new();
   serialize(interp(e, empty_vec.as_slice()))
}

fn main() {
   test();
}

fn test() {
  let mut fun_args: Vec<ExprC> = Vec::new();
  fun_args.push(ExprC::numC{n : 3});
  fun_args.push(ExprC::numC{n : 5});

  let mut fun_params: Vec<String> = Vec::new();
  fun_params.push("x".to_string());
  fun_params.push("y".to_string());
  //AppC tests
  
  assert_eq!(top_eval(ExprC::appC {fun: Box::new(ExprC::lamC {args: fun_params, body: Box::new(ExprC::binOpC {op: "+".to_string(), l: Box::new(ExprC::idC {s: "x".to_string()}), r: Box::new(ExprC::idC {s: "y".to_string()})})}), arg : fun_args}), "8");

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

  // if tests
  assert_eq!(top_eval(ExprC::ifC {test : Box::new(ExprC::boolC {b : false}), then : Box::new(ExprC::numC {n:5}), els : Box::new(ExprC::numC {n:3})}), "3");
  assert_eq!(top_eval(ExprC::ifC {test : Box::new(ExprC::boolC {b : true}), then : Box::new(ExprC::numC {n:5}), els : Box::new(ExprC::numC {n:3})}), "5");
  assert_eq!(top_eval(
        ExprC::ifC {test : Box::new(ExprC::binOpC {op : "<=".to_string(), l : Box::new(ExprC::numC { n : 3}), r : Box::new(ExprC::numC {n : 6})}),
                    then : Box::new(ExprC::numC {n:5}), 
                    els : Box::new(ExprC::numC {n:3})}), "5");
  assert_eq!(top_eval(
        ExprC::ifC {test : Box::new(ExprC::binOpC {op : "<=".to_string(), l : Box::new(ExprC::numC { n : 3}), r : Box::new(ExprC::numC {n : 6})}),
                    then : Box::new(ExprC::ifC {test : Box::new(ExprC::binOpC {op : "eq?".to_string(), 
                                                                                l : Box::new(ExprC::numC { n : 3}), 
                                                                                r : Box::new(ExprC::numC {n : 6})}),
                                                then : Box::new(ExprC::boolC {b: false}),
                                                 els : Box::new(ExprC::boolC {b:true})}),
                    els : Box::new(ExprC::numC {n:3})}
                    ), "True");
}
