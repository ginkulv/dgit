use postgres::{Client, NoTls, Error};

pub struct Table {
    pub domain: String,
    pub name: String,
}

pub fn db_init(url: &str) -> Client {
    return Client::connect(url, NoTls).unwrap();
}

pub fn get_tables(client: &mut Client) -> Vec<Table>  {
    let mut tables: Vec<Table> = Vec::new();
    for row in client.query("select distinct table_schema, table_name
        from information_schema.columns
        where table_schema not in ('information_schema', 'pg_catalog')", &[]).unwrap() {
            let table = Table {
                domain: row.get("table_schema"),
                name: row.get("table_name"),
            };
            tables.push(table);
    }
    return tables;
}
