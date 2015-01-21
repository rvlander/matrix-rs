use super::matrix::Matrix;
use num::traits::Zero;
use std::collections::HashMap;
use std::mem::replace;
use std::ops::Add;
use std::ops::Neg;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(PartialEq, Eq, Show)]
struct NaiveSparseMatrix<T> {
	m: usize,
	n: usize,
	values: HashMap<(usize,usize),T>
}

impl <T> Matrix<T>  for NaiveSparseMatrix<T> where T: Zero + PartialEq + Copy{



	// dont forget to return U
	fn element_wise_binary_op<F: Fn((&T, &T)) -> T>(self, rhs: NaiveSparseMatrix<T>,f : F) -> NaiveSparseMatrix<T> {
		assert_eq!(self.m, rhs.m);
		assert_eq!(self.n, rhs.n);
		
		let zero: T = Zero::zero();

		let mut remaining = rhs.values;
		let mut tmp: HashMap<(usize,usize),T> = self.values.iter().map(|a| (*a.0, match remaining.remove(a.0){
			Some(b) => f((a.1,&b)),
			None => f((a.1,&zero)),
		})).collect();
		for (indices, val) in remaining.iter() {
			tmp.insert(*indices, f((&zero, val)));
		} 
		NaiveSparseMatrix::new(self.m, self.n, tmp)	
	}

	// dont forget to return U
	fn element_wise_unary_op<F: Fn(&T) -> T>(self, f: F) -> NaiveSparseMatrix<T> {
		//let data = self.data.iter().map(f).collect();
		let zero: T = Zero::zero();
		let f0 = f(&zero);
		if f0.is_zero() {
			NaiveSparseMatrix::new(self.m, self.n,
				self.values.iter().map(|a| (*a.0,f(a.1))).collect())
		} else {
			let res = self.indices_iter().map(|a| (a, match self.values.get(&a) {
				Some(v) => f(v),
				None => f0,	
			})).collect();
			NaiveSparseMatrix::new(self.m, self.n, res)
		}
	}
	
	fn size(&self) -> (usize, usize) {
		return (self.m, self.n)
	}
}

impl <T> NaiveSparseMatrix<T> {
	fn new(m: usize, n: usize, values: HashMap<(usize,usize),T>) -> NaiveSparseMatrix<T> where T: Zero{
		let mut matrix = NaiveSparseMatrix {
			m: m,
			n: n,
			values: values
		};
		matrix.remove_zeros();
		//TODO: verify predicate here
		matrix
	}

	fn remove_zeros(&mut self) where T: Zero{
		self.values = replace(&mut self.values, HashMap::new()).into_iter().filter(|a| !a.1.is_zero()).collect()
	}
}


impl <T: Add<T, Output = T> + Copy> Add for NaiveSparseMatrix<T> where T: Zero + PartialEq {
	type Output = NaiveSparseMatrix<T>;

	fn add(self, rhs: NaiveSparseMatrix<T>) -> NaiveSparseMatrix<T> where T: Zero + PartialEq {
		self.__add(rhs)
	}
}

impl <T: Add<T, Output = T> + Copy> Add<T> for NaiveSparseMatrix<T> where T: Zero + PartialEq {
	type Output = NaiveSparseMatrix<T>;

	fn add(self, rhs: T) -> NaiveSparseMatrix<T> {
		self.scalar_add(rhs)
	}
}

impl <T: Sub<T, Output = T> + Copy> Sub for NaiveSparseMatrix<T> where T: Zero + PartialEq {
	type Output = NaiveSparseMatrix<T>;

	fn sub(self, rhs: NaiveSparseMatrix<T>) -> NaiveSparseMatrix<T> where T: Zero + PartialEq {
		self.__sub(rhs)
	}
}

impl <T: Sub<T, Output = T> + Copy> Sub<T> for NaiveSparseMatrix<T> where T: Zero + PartialEq {
	type Output = NaiveSparseMatrix<T>;

	fn sub(self, rhs: T) -> NaiveSparseMatrix<T>{
		self.scalar_sub(rhs)
	}
}

impl <T: Mul<T, Output = T> + Copy> Mul<T> for NaiveSparseMatrix<T> where T: Zero + PartialEq {
	type Output = NaiveSparseMatrix<T>;

	fn mul(self, rhs: T) -> NaiveSparseMatrix<T>{
		self.scalar_mul(rhs)
	}
}

impl <T: Div<T, Output = T> + Copy> Div<T> for NaiveSparseMatrix<T> where T: Zero + PartialEq {
	type Output = NaiveSparseMatrix<T>;

	fn div(self, rhs: T) -> NaiveSparseMatrix<T> {	
		self.scalar_div(rhs)
	}
}

impl <T: Neg<Output = T> + Copy> Neg for NaiveSparseMatrix<T> where T: Zero + PartialEq {
	type Output = NaiveSparseMatrix<T>;

	fn neg(self) -> NaiveSparseMatrix<T> {
		self.__neg()
	}
}

#[cfg(test)]
mod test {
	use super::super::matrix::Matrix;
	use super::NaiveSparseMatrix;
	use std::collections::HashMap;

	#[test]
	fn test_add (){
		let mut hm1 = HashMap::new();
		hm1.insert((1,2), 10);
		hm1.insert((2,1), 5);
		hm1.insert((0,0), 7);

		let mut hm2 = HashMap::new();
		hm2.insert((0,0), 14);
		hm2.insert((2,2), 20);

		let mut hm3 = HashMap::new();
		hm3.insert((0,0), 21);
		hm3.insert((1,2), 10);
		hm3.insert((2,1), 5);
		hm3.insert((2,2), 20);
	
		let m1 = NaiveSparseMatrix::new(3, 3, hm1);
		let m2 = NaiveSparseMatrix::new(3, 3, hm2);
		let m3 = NaiveSparseMatrix::new(3, 3, hm3);
		assert_eq!(m1+m2, m3);
	}

	#[test]
	fn test_add_scalar (){
		let mut hm1 = HashMap::new();
		hm1.insert((1,2), 10);
		hm1.insert((2,1), 5);
		hm1.insert((0,0), 7);

		let mut hm2 = HashMap::new();
		hm2.insert((0,0), 13);
		hm2.insert((0,1), 6);
		hm2.insert((0,2), 6);
		hm2.insert((1,0), 6);
		hm2.insert((1,1), 6);
		hm2.insert((1,2), 16);
		hm2.insert((2,0), 6);
		hm2.insert((2,1), 11);
		hm2.insert((2,2), 6);

		
		let m1 = NaiveSparseMatrix::new(3, 3, hm1);
		let m4 = NaiveSparseMatrix::new(3, 3, hm2);
		assert_eq!(m1+6, m4);
	}

	#[test]
	fn test_sub (){
		let mut hm1 = HashMap::new();
		hm1.insert((1,2), 10);
		hm1.insert((2,1), 5);
		hm1.insert((0,0), 7);

		let mut hm2 = HashMap::new();
		hm2.insert((0,0), 14);
		hm2.insert((2,2), 20);

		let mut hm3 = HashMap::new();
		hm3.insert((0,0), -7);
		hm3.insert((1,2), 10);
		hm3.insert((2,1), 5);
		hm3.insert((2,2), -20);
	
		let m1 = NaiveSparseMatrix::new(3, 3, hm1);
		let m2 = NaiveSparseMatrix::new(3, 3, hm2);
		let m3 = NaiveSparseMatrix::new(3, 3, hm3);
		assert_eq!(m1-m2, m3);
	}

	#[test]
	fn test_sub_scalar (){
		let mut hm1 = HashMap::new();
		hm1.insert((1,2), 10);
		hm1.insert((2,1), 5);
		hm1.insert((0,0), 7);

		let mut hm2 = HashMap::new();
		hm2.insert((0,0), -5);
		hm2.insert((0,1), -12);
		hm2.insert((0,2), -12);
		hm2.insert((1,0), -12);
		hm2.insert((1,1), -12);
		hm2.insert((1,2), -2);
		hm2.insert((2,0), -12);
		hm2.insert((2,1), -7);
		hm2.insert((2,2), -12);
		
		let m1 = NaiveSparseMatrix::new(3, 3, hm1);
		let m4 = NaiveSparseMatrix::new(3, 3, hm2);
		assert_eq!(m1-12, m4);
	}

	#[test]
	fn test_mul_scalar (){
		let mut hm1 = HashMap::new();
		hm1.insert((1,2), 10);
		hm1.insert((2,1), 5);
		hm1.insert((0,0), 7);

		let mut hm2 = HashMap::new();
		hm2.insert((0,0), 14);
		hm2.insert((1,2), 20);
		hm2.insert((2,1), 10);
		
		let m1 = NaiveSparseMatrix::new(3, 3, hm1);
		let m4 = NaiveSparseMatrix::new(3, 3, hm2);
		assert_eq!(m1*2, m4);
	}

	#[test]
	fn test_element_wise_mul (){
		let mut hm1 = HashMap::new();
		hm1.insert((1,2), 10);
		hm1.insert((2,1), 5);
		hm1.insert((0,0), 7);

		let mut hm2 = HashMap::new();
		hm2.insert((0,0), 2);
		hm2.insert((2,2), 20);

		let mut hm3 = HashMap::new();
		hm3.insert((0,0), 14);
	
		let m1 = NaiveSparseMatrix::new(3, 3, hm1);
		let m2 = NaiveSparseMatrix::new(3, 3, hm2);
		let m3 = NaiveSparseMatrix::new(3, 3, hm3);
		assert_eq!(m1.element_wise_multiply(m2), m3);
	}

	#[test]
	fn test_neg (){
		let mut hm1 = HashMap::new();
		hm1.insert((1,2), 10);
		hm1.insert((2,1), 5);
		hm1.insert((0,0), 7);

		let mut hm2 = HashMap::new();
		hm2.insert((0,0), -7);
		hm2.insert((1,2), -10);
		hm2.insert((2,1), -5);
	
		let m1 = NaiveSparseMatrix::new(3, 3, hm1);
		let m4 = NaiveSparseMatrix::new(3, 3, hm2);
		assert_eq!(-m1, m4);
	}

	#[test]
	fn test_scalar_div (){
		let mut hm1 = HashMap::new();
		hm1.insert((1,2), 10f64);
		hm1.insert((2,1), 5f64);
		hm1.insert((0,0), 7f64);

		let mut hm2 = HashMap::new();
		hm2.insert((0,0), 3.5);
		hm2.insert((1,2), 5.0);
		hm2.insert((2,1), 2.5);
	
		let m1 = NaiveSparseMatrix::new(3, 3, hm1);
		let m2 = NaiveSparseMatrix::new(3, 3, hm2);
		assert_eq!(m1/2.0, m2);
	}

	#[test]
	fn test_indices_iter () {
		let m1: NaiveSparseMatrix<f32>= NaiveSparseMatrix::new(10, 10, HashMap::new());
		let (m,n) = m1.size();
		let mut iter = m1.indices_iter();
		for i in (0..m) {
			for j in (0..n) {
				let a = iter.next();
				match a {
					Some((u,v)) => println!("{} {}", u, v),
					_ => println!("None"),
				}	
				assert_eq!(Some((i,j)), a);
			}
		}
		
		assert_eq!(None, iter.next())
	}
}