pub trait Matrix<T> {
	fn element_wise_binary_op<F: Fn((&T, &T)) -> T>(self, rhs: Self,f : F) -> Self;

	// dont forget to return U
	fn element_wise_unary_op<F: Fn(&T) -> T>(self, f: F) -> Self;
}
