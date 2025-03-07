extern crate postgres;

use postgres::{Client, NoTls};

pub fn get_client(
    host: String,
    user: String,
    password: String,
    port: String,
    dbname: String,
) -> Client {
    match Client::connect(
        format!(
            "host={} user={} password={} port={} dbname={}",
            host, user, password, port, dbname
        )
        .as_str(),
        NoTls,
    ) {
        Err(why) => panic!("error while creating the sql client: {}", why),
        Ok(value) => value,
    }
}

pub fn to_db(mut client: Client) -> Result<(), Box<dyn std::error::Error>> {
    let rows = match client.query("select count(1) from poi;", &[]) {
        Err(why) => panic!("error reading the table {}", why),
        Ok(value) => value,
    };
    for row in rows {
        let value: i64 = row.get("count");
        println!("row: {}", value);
    }
    Ok(())
}
