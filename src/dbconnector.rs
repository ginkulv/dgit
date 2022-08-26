use postgres::{Client, NoTls, Error};

struct Table {
    domain: String,
    name: String,
}

pub fn db_init(url: &str) -> Client {
    return Client::connect(url, NoTls).unwrap();
}
// -> Result<Vec<Table>, Error>
pub fn get_tables(client: Client)  {
    for row in client.query("select distinct table_schema, table_name
        from information_schema.columns
        where table_schema not in ('information_schema', 'pg_catalog')", &[]) {
            let domain: &str = row.get(0).unwrap();
            let table = Table {
                domain: row.get(0),
                name: row.get(1),
            };
    }
}
