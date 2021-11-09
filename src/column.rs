
/// Describes a column in a database
pub trait Column {}

macro_rules! column {
	($Self:ident) => {
		#[doc = concat!("The column `", stringify!($Self), "`")]
		#[derive(Debug)]
		pub struct $Self<T>(pub T);

		impl<T> Column for $Self<T> {}
	};
	($($Self:ident),*) => {$(
		column! { $Self }
	)*}
}

column! { A, B, C, D, E, F, G, H, I, J, K, L }
