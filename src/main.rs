
mod name;

use name::*;

use core::ops::RangeFrom;



#[derive(Debug)]
struct Site {
	url: Unique<String>,
	title: String,
}
#[derive(Debug)]
struct Link {
	from: Foreign<Site>,
	to: Foreign<Site>,
	text: String,
}

impl InsertIntoDatabase<(A<Column<Site>>, B<Column<Link>>,)> for A<Site> {
	fn insert_into_database(self, (A(ref mut a), ..): &mut (A<Column<Site>>, B<Column<Link>>,)) -> Result<(), Error> {
		self.0.url.check_constraint(a)?;

		a.insert(self.0);

		Ok(())
	}
}

/// analogous to `SQL` `FOREIGN KEY`
#[derive(Debug)]
struct Foreign<T>(T);

/// analogous to `SQL` `UNIQUE`
#[derive(Debug)]
struct Unique<T>(T);

impl<T: Eq> Unique<T> {
	fn check_constraint(&self, other: &Iter<T>) -> Result<(), Error> {
		if column.iter().all(|other| other.0 != self.0) {
			Ok(())
		} else {
			Err(String::from("Unique constraint violated"))
		}
	}
}

type Error = String;

trait InsertIntoDatabase<D>: Name {
	fn insert_into_database(self, _: &mut D) -> Result<(), Error>;
}


#[derive(Debug)]
struct Entry<T> {
	key: u32,
	val: T,
}

#[derive(Debug)]
struct Column<T> {
	unique_key_factory: RangeFrom<u32>,
	data: Vec<Option<Entry<T>>>,
}

impl<T> Column<T> {
	fn new() -> Self {
		Self {
			unique_key_factory: 0..,
			data: Vec::new(),
		}
	}
	fn first_free_mut(&mut self) -> &mut Option<Entry<T>> {
		if let Some(slot) = self.data.iter_mut().find(|option| option.is_some()) {
			slot
		} else {
			self.data.push(None);

			self.data.last_mut().expect("unreachable")
		}
	}
	fn insert(&mut self, val: T) -> u32 {
		let key = self.unique_key_factory.next().expect("bitspace exhausted");

		*self.first_free_mut() = Some(Entry { key, val });

		key
	}
}

fn main() {
 	let mut database = (
		A(Column::<Site>::new()),
		B(Column::<Link>::new()),
	);

	let site = Site { url: Unique(String::from("https://google.com")), title: String::from("Google") };

	A(site).insert_into_database(&mut database);

	println!("{:?}", database);
}
