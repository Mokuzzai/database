use core::ops::RangeFrom;

struct Row<T> {
	ident: u32,
	value: T,
}

struct Column<T> {
	cuidf: RangeFrom<u32>, // column unique identifier factory
	items: Vec<Option<Row<T>>>,
}

impl<T> Column<T> {
	fn new() -> Self {
		Self {
			cuidf: 0..,
			items: Vec::new(),
		}
	}
	fn next_cuid(&mut self) -> u32 {
		self.cuidf.next().expect("bitspace exhausted")
	}
}

struct Site {
	url: String, // unique,
	title: String,
}

impl Site {}

struct Link {
	from: u32, // foreign key `Site`
	to: u32,   // foreign key `Site`
	text: String,
}

struct LinkRef<'a> {
	from: &'a Site,
	to: &'a Site,
	text: &'a String,
}

impl Link {
	fn iter(database: &Database) -> impl Iterator<Item = LinkRef> {
		database.link.items.iter().flat_map(|slot| {
			let Row {
				value: Link {
					from: from_ident,
					to: to_ident,
					ref text,
				},
				..
			} = *slot.as_ref()?;

			let mut sites = database.site.items.iter();

			let mut from = None;
			let mut to = None;

			for row in database.site.items.iter().flat_map(|slot| slot.as_ref()) {
				if row.ident == from_ident {
					from = Some(&row.value);

					if to.is_some() {
						break;
					}
				} else if row.ident == to_ident {
					to = Some(&row.value);

					if from.is_some() {
						break;
					}
				}
			}

			Some(LinkRef {
				from: from.expect("foreign key constraint violated"),
				to: to.expect("foreign key constraint violated"),
				text,
			})
		})
	}
}

struct Database {
	site: Column<Site>,
	link: Column<Link>,
}

impl Database {
	fn new() -> Self {
		Self {
			site: Column::new(),
			link: Column::new(),
		}
	}
}

fn main() {
	let mut database = Database::new();
}
