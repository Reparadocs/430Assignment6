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

struct ifC {
   test: ExprC,
   then: ExprC,
   els: ExprC
}

struct appC {
   fun: ExprC,
   arg: Vec<ExprC>
}

struct binOpC {
   op: String,
   l: ExprC,
   r: ExprC
}

struct lamC {
   args: Vec<ExprC>,
   body: ExprC
}

impl ExprC for numC {
   fn interp(&self) -> i32 {
      self.n 
   }
}

fn get_num(e: &ExprC) {
   println!("Num: {}", e.interp());
}

fn main() {
   println!("Hello World!");
   let test_num = numC { n : 1 };
   get_num(&test_num);
}
