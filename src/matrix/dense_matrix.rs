#![allow(dead_code)]


use std::iter::repeat;

#[deriving(PartialEq, Eq, Show)]
pub struct DenseMatrix<T> {
	m: uint,
	n: uint,
	data: Vec<T>
}

impl <T> DenseMatrix<T> {
	fn from_elem(m: uint, n: uint, init: T) -> DenseMatrix<T>
	        where T: Clone {
		let data = repeat(init).take(n*m).collect();
		DenseMatrix {
		    m: m, 
		    n: n, 
		    data: data
		}
	}

	fn element_wise_binary_op<U, F: Fn((&T, &T)) -> U>(self, rhs: DenseMatrix<T>,f : F) -> DenseMatrix<U> {
		assert_eq!(self.m, rhs.m);
		assert_eq!(self.n, rhs.n);
		DenseMatrix {
			m: self.m,
			n: self.n,
			data: self.data.iter().zip(rhs.data.iter()).map(f).collect()
		}
	}

	fn element_wise_unary_op<U, F: Fn(&T) -> U>(self, f: F) -> DenseMatrix<U> {
		DenseMatrix {
			m: self.m,
			n: self.n,
			data: self.data.iter().map(f).collect()
		}
	}

}

impl <T: Mul<T,T> + Copy> DenseMatrix<T> {
	fn element_wise_multiply(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		self.element_wise_binary_op(rhs,|(a, b)| *a * *b)
	}  
}

impl <T: Add<T, T> + Copy> Add<DenseMatrix<T>, DenseMatrix<T>> for DenseMatrix<T> {
	fn add(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		self.element_wise_binary_op(rhs,|(a, b)| *a + *b)
	}
}

impl <T: Add<T, T> + Copy> Add<T, DenseMatrix<T>> for DenseMatrix<T> {
	fn add(self, rhs: T) -> DenseMatrix<T> {
		self.element_wise_unary_op(|a| rhs + *a)
	}
}

/*impl <T: Add<T, T> + Copy> Add<DenseMatrix<T>, DenseMatrix<T>> for T {
	fn add(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		rhs + self;
	}
}*/

impl <T: Sub<T, T> + Copy> Sub<DenseMatrix<T>, DenseMatrix<T>> for DenseMatrix<T> {
	fn sub(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		self.element_wise_binary_op(rhs,|(a, b)| *a - *b)
	}
}

impl <T: Sub<T, T> + Copy> Sub<T, DenseMatrix<T>> for DenseMatrix<T> {
	fn sub(self, rhs: T) -> DenseMatrix<T> {
		self.element_wise_unary_op(|a| *a - rhs)
	}
}

/*impl <T: Sub<T, T> + Copy> Sub<DenseMatrix<T>, DenseMatrix<T>> for T {
	fn sub(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		rhs - self;
	}
}*/
	
impl <T: Mul<T, T> + Copy> Mul<T, DenseMatrix<T>> for DenseMatrix<T> {
	fn mul(self, rhs: T) -> DenseMatrix<T> {
		self.element_wise_unary_op(|a| rhs * *a)
	}
}

/*impl <T: Mul<T, T> + Copy> Mul<DenseMatrix<T>, DenseMatrix<T>> for T {
	fn mul(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		rhs * self;
	}
}*/

impl <T: Div<T, T> + Copy> Div<T, DenseMatrix<T>> for DenseMatrix<T> {
	fn div(self, rhs: T) -> DenseMatrix<T> {
		self.element_wise_unary_op(|a| *a / rhs)
	}
}

/*impl <T: Div<T, T> + Copy> Div<DenseMatrix<T>, DenseMatrix<T>> for T {
	fn div(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		rhs * self;
	}
}*/

impl <T: Neg<T> + Copy> Neg< DenseMatrix<T>> for DenseMatrix<T> {
	fn neg(self) -> DenseMatrix<T> {
		self.element_wise_unary_op(|a| - *a)
	}
}




#[cfg(test)]
mod test {
	use super::DenseMatrix;

	#[test]
	fn test_from_elem () {
		let m1 = DenseMatrix::from_elem(10,10,10i);
		let m2 = DenseMatrix::from_elem(10,10,10i);
		assert_eq!(m1, m2);

		let m1 = DenseMatrix::from_elem(10,20,10i);
		let m2 = DenseMatrix::from_elem(10,10,10i);
		assert!(m1 != m2);
	}

	#[test]
	fn test_add (){
		let m1 = DenseMatrix::from_elem(10,10,10i);
		let m2 = DenseMatrix::from_elem(10,10,10i);
		let m3 = DenseMatrix::from_elem(10,10,10i);
		let m4 = DenseMatrix::from_elem(10,10,30i);
		assert_eq!(m1+m2+m3, m4);
	}

	#[test]
	fn test_add_scalar (){
		let m1 = DenseMatrix::from_elem(10,10,10i);
		let m2 = DenseMatrix::from_elem(10,10,14i);
		assert_eq!(m1+4, m2);
	}

	#[test]
	fn test_sub (){
		let m1 = DenseMatrix::from_elem(10,10,10i);
		let m2 = DenseMatrix::from_elem(10,10,10i);
		let m3 = DenseMatrix::from_elem(10,10,10i);
		let m4 = DenseMatrix::from_elem(10,10,-10i);
		assert_eq!(m1-m2-m3, m4);
	}

	#[test]
	fn test_sub_scalar (){
		let m1 = DenseMatrix::from_elem(10,10,10i);
		let m2 = DenseMatrix::from_elem(10,10,6i);
		assert_eq!(m1-4, m2);
	}

	#[test]
	fn test_mul_scalar (){
		let m1 = DenseMatrix::from_elem(10,10,10i);
		let m4 = DenseMatrix::from_elem(10,10,60i);
		assert_eq!(m1*6, m4);
	}

	#[test]
	fn test_element_wise_mul (){
		let m1 = DenseMatrix::from_elem(10,10,10i);
		let m2 = DenseMatrix::from_elem(10,10,20i);
		let m4 = DenseMatrix::from_elem(10,10,200i);
		assert_eq!(m1.element_wise_multiply(m2), m4);
	}

	#[test]
	fn test_neg (){
		let m1 = DenseMatrix::from_elem(10,10,10i);
		let m2 = DenseMatrix::from_elem(10,10,-10i);
		assert_eq!(-m1, m2);
	}

	#[test]
	fn test_scalar_div (){
		let m1 = DenseMatrix::from_elem(10,10,8f64);
		let m2 = DenseMatrix::from_elem(10,10,2f64);
		assert_eq!(m1/4.0, m2);
	}
}