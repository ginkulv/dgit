use postgres::{Client, NoTls, Error};
use postgres_native_tls::MakeTlsConnector;
use native_tls::TlsConnector;

pub struct Table {
    pub domain: String,
    pub name: String,
}

impl Table {
    pub fn new(domain: String, name: String) -> Self {
        Table {
            domain,
            name
        }
    }
}

pub fn db_init(url: &str) -> Client {
    let connector = TlsConnector::builder().build().expect("An error occured");
    let connector = MakeTlsConnector::new(connector);
    return Client::connect(url, connector).expect("An error occured");
}

pub fn get_tables(client: &mut Client) -> Vec<Table>  {
    let mut tables: Vec<Table> = Vec::new();
    for row in client.query("select distinct table_schema, table_name
        from information_schema.columns
        where table_schema not in ('information_schema', 'pg_catalog')", &[]).unwrap() {
            let table = Table::new(row.get("table_schema"), row.get("table_name"));
            tables.push(table);
    }
    return tables;
}
