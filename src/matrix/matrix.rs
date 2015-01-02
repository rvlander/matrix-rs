pub trait Matrix<T> {
	fn element_wise_binary_op<F: Fn((&T, &T)) -> T>(self, rhs: Self,f : F) -> Self;

	// dont forget to return U
	fn element_wise_unary_op<F: Fn(&T) -> T>(self, f: F) -> Self;

	fn __add(self, rhs: Self) -> Self where T: Add<T,T> + Copy {
		self.element_wise_binary_op(rhs,|(a, b)| *a + *b)
	}

	fn scalar_add(self, rhs: T)-> Self where T: Add<T,T> + Copy {
		self.element_wise_unary_op(|a| rhs + *a)
	} 

	fn element_wise_multiply(self, rhs: Self) -> Self 
		where T: Mul<T,T> + Copy{
		self.element_wise_binary_op(rhs,|(a, b)| *a * *b)
	}  

	fn __sub(self, rhs: Self) -> Self where T: Sub<T, T> + Copy {
		self.element_wise_binary_op(rhs,|(a, b)| *a - *b)
	}

	fn scalar_sub(self, rhs: T) -> Self where T: Sub<T, T> + Copy {
		self.element_wise_unary_op(|a| *a - rhs)
	}

	fn scalar_mul(self, rhs: T) -> Self where T: Mul<T, T> + Copy {
		self.element_wise_unary_op(|a| rhs * *a)
	}

	fn scalar_div(self, rhs: T) -> Self where T: Div<T, T> + Copy {
		self.element_wise_unary_op(|a| *a / rhs)
	}

	fn __neg(self) -> Self where T: Neg<T> + Copy {
		self.element_wise_unary_op(|a| - *a)
	}
}
