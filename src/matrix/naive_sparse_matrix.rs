use super::matrix::Matrix;
use num::traits::Zero;
use std::collections::HashMap;
use std::mem::replace;

#[derive(PartialEq, Eq, Show)]
struct NaiveSparseMatrix<T> {
	m: usize,
	n: usize,
	values: HashMap<(usize,usize),T>
}

impl <T> Matrix<T>  for NaiveSparseMatrix<T> where T: Zero + PartialEq{



	// dont forget to return U
	fn element_wise_binary_op<F: Fn((&Ts, &T)) -> T>(self, rhs: NaiveSparseMatrix<T>,f : F) -> NaiveSparseMatrix<T> {
		assert_eq!(self.m, rhs.m);
		assert_eq!(self.n, rhs.n);
		//let data = self.data.iter().zip(rhs.data.iter()).map(f).collect();
		NaiveSparseMatrix::new(self.m, self.n, HashMap::new())	
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
			//TODO
			NaiveSparseMatrix::new(self.m, self.n, HashMap::new())
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