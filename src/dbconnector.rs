use postgres::Client;
use postgres_native_tls::MakeTlsConnector;
use native_tls::TlsConnector;

pub enum EntityType {
    TABLE,
    NONE
}

pub struct Entity {
    pub domain: String,
    pub name: String,
    pub entity_type: EntityType,
}

impl Entity {
    pub fn new(domain: String, name: String, entity_type: EntityType) -> Self {
        Entity {
            domain,
            name,
            entity_type
        }
    }
}

pub fn db_init(name: &str, password: &str, url: &str, dbname: &str) -> Client {
    let conn_str = format!("postgresql://{}:{}@{}/{}", name, &password, &url, &dbname);
    let connector = TlsConnector::builder().build().expect("An error occured");
    let connector = MakeTlsConnector::new(connector);
    return Client::connect(&conn_str, connector).expect("An error occured");
}

pub fn get_entities(client: &mut Client) -> Vec<Entity>  {
    let mut entities: Vec<Entity> = Vec::new();
    for row in client.query("select distinct table_schema, table_name
        from information_schema.columns
        where table_schema not in ('information_schema', 'pg_catalog')", &[]).unwrap() {
            let entity = Entity::new(row.get("table_schema"), row.get("table_name"), EntityType::TABLE);
            entities.push(entity);
    }
    return entities;
}
