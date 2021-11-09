
/// Describes a column in a database
pub trait Name {}

macro_rules! name {
	($Self:ident) => {
		#[doc = concat!("The name `", stringify!($Self), "`")]
		#[derive(Debug)]
		pub struct $Self<T>(pub T);

		impl<T> Name for $Self<T> {}
	};
	($($Self:ident),*) => {$(
		name! { $Self }
	)*}
}

name! { A, B, C, D, E, F, G, H, I, J, K, L }
