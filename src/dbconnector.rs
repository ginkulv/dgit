use postgres::{Client, Error};
use postgres_native_tls::MakeTlsConnector;
use native_tls::TlsConnector;
use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub name: String,
    pub password: String,
    pub url: String,
    pub dbname: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Column {
    pub name: String,
    pub data_type: String,
}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.data_type == other.data_type && self.name == other.name
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash)]
pub struct Entity {
    pub schema: String,
    pub name: String,
    pub columns: Vec<Column>,
    pub exists: bool,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.schema == other.schema && self.name == other.name && self.columns == other.columns
    }
}

pub fn db_init(creds: &Credentials) -> Result<Client, Error> {
    let conn_str = format!("postgresql://{}:{}@{}/{}", creds.name, creds.password, creds.url, creds.dbname);
    let connector = TlsConnector::builder().build().expect("TlsConnector built successfully");
    let connector = MakeTlsConnector::new(connector);
    Client::connect(&conn_str, connector)
}

pub fn get_entities(client: &mut Client) -> Vec<Entity>  {
    let mut entities: Vec<Entity> = Vec::new();
    let mut query = r#"
        select schemaname, tablename
        from pg_catalog.pg_tables
        where schemaname not in ('information_schema', 'pg_catalog')
    "#;
    for row in client.query(query, &[]).unwrap() {
        let name: String = row.get("tablename");
        let schema: String = row.get("schemaname");

        query = r#"
            select column_name, data_type
            from INFORMATION_SCHEMA.columns
            where table_schema = $1 and table_name = $2
            order by column_name
        "#;

        let mut columns: Vec<Column> = Vec::new();

        for column in client.query(query, &[&schema, &name]).unwrap() {
            columns.push(Column {
                data_type: column.get("data_type"),
                name: column.get("column_name"),
            });
        }
        let entity = Entity {
            name,
            schema,
            columns,
            exists: true,
        };
        entities.push(entity);
    }
    entities
}
