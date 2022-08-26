use postgres::{Client, NoTls, Error};

struct Table {
    domain: String,
    name: String,
}

pub fn db_init(url: &str) -> Client {
    return Client::connect(url, NoTls).unwrap();
}

pub fn get_tables(client: &mut Client)  {
    for row in client.query("select distinct table_schema, table_name
        from information_schema.columns
        where table_schema not in ('information_schema', 'pg_catalog')", &[]).unwrap() {
            let table = Table {
                domain: row.get("table_schema"),
                name: row.get("table_name"),
            };

            println!("{:?}", table.domain);
            println!("{:?}", table.name);
    }
}
