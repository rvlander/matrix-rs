use std::iter::repeat;
use super::matrix::Matrix;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(PartialEq, Eq, Show)]
struct DenseMatrix<T> {
	m: usize,
	n: usize,
	data: Vec<T>
}

impl <T> Matrix<T>  for DenseMatrix<T> {

	// dont forget to return U
	fn element_wise_binary_op<F: Fn((&T, &T)) -> T>(self, rhs: DenseMatrix<T>,f : F) -> DenseMatrix<T> {
		assert_eq!(self.m, rhs.m);
		assert_eq!(self.n, rhs.n);
		let data = self.data.iter().zip(rhs.data.iter()).map(f).collect();
		DenseMatrix::new(self.m, self.n, data)	
	}

	// dont forget to return U
	fn element_wise_unary_op<F: Fn(&T) -> T>(self, f: F) -> DenseMatrix<T> {
		let data = self.data.iter().map(f).collect();
		DenseMatrix::new(self.m, self.n, data)
	}

	fn size(&self) -> (usize, usize) {
		return (self.m, self.n)
	}


}

impl <T> DenseMatrix<T> {
	fn new(m: usize, n: usize, data: Vec<T>) -> DenseMatrix<T> {
		let matrix = DenseMatrix {
			m: m,
			n: n,
			data: data
		};
		assert_eq!(matrix.m * matrix.n, matrix.data.len());
		matrix
	}

	fn from_elem(m: usize, n: usize, init: T) -> DenseMatrix<T>
	        where T: Clone {
		let data = repeat(init).take(n*m).collect();
		DenseMatrix::new(m, n, data)
	}
}

impl <T: Add<T, Output = T> + Copy> Add for DenseMatrix<T> {
	type Output = DenseMatrix<T>;

	fn add(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		self.__add(rhs)
	}
}

impl <T: Add<T, Output = T> + Copy> Add<T> for DenseMatrix<T> {
	type Output = DenseMatrix<T>;

	fn add(self, rhs: T) -> DenseMatrix<T> {
		self.scalar_add(rhs)
	}
}

impl <T: Sub<T, Output = T> + Copy> Sub for DenseMatrix<T> {
	type Output = DenseMatrix<T>;

	fn sub(self, rhs: DenseMatrix<T>) -> DenseMatrix<T> {
		self.__sub(rhs)
	}
}

impl <T: Sub<T, Output = T> + Copy> Sub<T> for DenseMatrix<T> {
	type Output = DenseMatrix<T>;

	fn sub(self, rhs: T) -> DenseMatrix<T> {
		self.scalar_sub(rhs)
	}
}

impl <T: Mul<T, Output = T> + Copy> Mul<T> for DenseMatrix<T> {
	type Output = DenseMatrix<T>;

	fn mul(self, rhs: T) -> DenseMatrix<T> {
		self.scalar_mul(rhs)
	}
}

impl <T: Div<T, Output = T> + Copy> Div<T> for DenseMatrix<T> {
	type Output = DenseMatrix<T>;

	fn div(self, rhs: T) -> DenseMatrix<T> {	
		self.scalar_div(rhs)
	}
}

impl <T: Neg<Output = T> + Copy> Neg for DenseMatrix<T> {
	type Output = DenseMatrix<T>;

	fn neg(self) -> DenseMatrix<T> {
		self.__neg()
	}
}




#[cfg(test)]
mod test {
	use super::super::matrix::Matrix;
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