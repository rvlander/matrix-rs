use std::ops::Add;
use std::ops::Neg;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

pub trait Matrix<T> :Sized{

	fn element_wise_binary_op<F: Fn((&T, &T)) -> T>(self, rhs: Self,f : F) -> Self;

	// dont forget to return U
	fn element_wise_unary_op<F: Fn(&T) -> T>(self, f: F) -> Self;

	fn size(&self) -> (uint, uint);

	fn __add(self, rhs: Self) -> Self where T: Add<T, Output = T> + Copy {
		self.element_wise_binary_op(rhs,|(a, b)| *a + *b)
	}

	fn scalar_add(self, rhs: T)-> Self where T: Add<T, Output = T> + Copy {
		self.element_wise_unary_op(|a| rhs + *a)
	} 

	fn element_wise_multiply(self, rhs: Self) -> Self
		where T: Mul<T, Output = T> + Copy{
		self.element_wise_binary_op(rhs,|(a, b)| *a * *b)
	}  

	fn __sub(self, rhs: Self) -> Self where T: Sub<T, Output = T> + Copy {
		self.element_wise_binary_op(rhs,|(a, b)| *a - *b)
	}

	fn scalar_sub(self, rhs: T) -> Self where T: Sub<T, Output = T> + Copy {
		self.element_wise_unary_op(|a| *a - rhs)
	}

	fn scalar_mul(self, rhs: T) -> Self where T: Mul<T, Output = T> + Copy {
		self.element_wise_unary_op(|a| rhs * *a)
	}

	fn scalar_div(self, rhs: T) -> Self where T: Div<T, Output = T> + Copy {
		self.element_wise_unary_op(|a| *a / rhs)
	}

	fn __neg(self) -> Self where T: Neg<Output = T> + Copy {
		self.element_wise_unary_op(|a| - *a)
	}
}

#[derive(Copy)]
pub struct IndicesIter {
	m: uint,
	n: uint
}

/*impl Iterator<(uint, uint)> for IndicesIter {
	fn next(&mut self) -> Option<A> {

	}
}*/

