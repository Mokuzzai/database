
mod column;

use column::*;

#[derive(Debug)]
struct Foreign<T>(T);

#[derive(Debug)]
struct Site {
	url: String,
	title: String,
}

#[derive(Debug)]
struct Link {
	from: Foreign<Site>,
	to: Foreign<Site>,
	text: String,
}

trait InsertInto<D>: Column {
	fn insert_into(self, _: &mut D);
}

fn raw_insert_into<T>(t: T, vec: &mut Vec<T>) {
	todo!()
}

impl<_A, _B> InsertInto<(A<Vec<_A>>, B<Vec<_B>>,)> for A<_A> {
	fn insert_into(self, (A(ref mut a), ..): &mut (A<Vec<_A>>, B<Vec<_B>>,)) {
		raw_insert_into(self.0, a)
	}
}


fn main() {
 	let mut database = (
		A(Vec::<Site>::new()),
		B(Vec::<Link>::new()),
	);

	let site = Site { url: String::from("https://google.com"), title: String::from("Google") };

	A(site).insert_into(&mut database);

	println!("{:?}", database);
}
