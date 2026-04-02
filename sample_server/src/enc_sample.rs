use config::CentraleConfig;
use dir_and_db_pool::db::{
    db_file::db_file,
    encrypted::{create_secret_db, get_secret_db},
};

fn main() {
    let file_path = db_file("test_server", CentraleConfig::DB_FOLDER).unwrap();
    let path = file_path.to_str().unwrap();
    // CREATE DB
    create_secret_db(&path, "pass").unwrap();
    // GET CONNECDTION
    let conn = get_secret_db(&path, "pass").unwrap();

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS notes (id INTEGER PRIMARY KEY, body TEXT);
    ",
    )
    .unwrap();

    conn.execute(
        "INSERT INTO notes (body) VALUES (?1)",
        ["hello encrypted world"],
    )
    .unwrap();

    let mut stmt = conn.prepare("SELECT id, body FROM notes").unwrap();
    let rows = stmt
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .unwrap();

    for row in rows {
        println!("{:?}", row);
    }

    /*
    match start_server(db) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("err, {}", err)
        }
    }
     */
}
