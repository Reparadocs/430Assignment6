fn interp(expression: ExcrC) {
	match expression {
		numC => expression.n;
		boolC => expression.b;
		idC => expression.s;
	}	
}

fn main() {
	
}
