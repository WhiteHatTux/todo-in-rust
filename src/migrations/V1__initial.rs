use barrel::{types, Migration, backend::Sqlite};

pub fn migration() -> String {
    let mut m = Migration::new();

    m.create_table("todo", |t| {
        t.add_column("id", types::text());
        t.add_column("title", types::varchar(255));
        t.add_column("content", types::varchar(1000));
    });

    m.make::<Sqlite>()
}
