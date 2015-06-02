trait ExprC {
   fn interp(&self) -> i32;
}

struct numC {
   n: i32
}

struct boolC {
   b: bool
}

struct idC {
   s: String
}

struct ifC<'a> {
   test: &'a (ExprC + 'a), 
   then: &'a (ExprC + 'a),
   els: &'a (ExprC + 'a)
}

struct appC<'a> {
   fun: &'a (ExprC + 'a),
   arg: Vec<ExprC>
}

struct binOpC<'a> {
   op: String,
   l: &'a (ExprC + 'a),
   r: &'a (ExprC + 'a)
}

struct lamC<'a> {
   args: Vec<ExprC>,
   body: &'a (ExprC + 'a)
}

impl ExprC for numC {
   fn interp(&self) -> i32 {
      self.n 
   }
}

impl<'a> ExprC for binOpC {
   fn interp(&self) -> i32 {
      let left = self.l.interp();
      let right = self.r.interp();

      match self.op {
         "+" => left + right,
         "-" => left - right,
         "*" => left * right,
         "/" => left / right,
      }
   }
}

fn interp(e: &ExprC) {
   println!("Num: {}", e.interp());
}

fn main() {
   println!("Hello World!");
   let test_num = binOpC {op : "+", l : numC { n : 5 }, r : numC { n : 3} };
   interp(&test_num);
}
