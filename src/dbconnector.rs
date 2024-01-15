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
pub struct Entity {
    pub domain: String,
    pub name: String,
    pub exists: bool,
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.name == other.name
    }
}

impl Entity {
    pub fn to_string(&self) -> String {
        format!("{}.{}", self.domain, self.name)
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
    let query = r#"
        select schemaname, tablename
        from pg_catalog.pg_tables
        where schemaname not in ('information_schema', 'pg_catalog')
    "#;
    for row in client.query(query, &[]).unwrap() {
            let entity = Entity {
                name: row.get("tablename"),
                domain: row.get("schemaname"),
                exists: true
            };
            entities.push(entity);
        }
    return entities;
}
