mod error;

use crate::error::TestSuiteError;
use dir_and_db_pool::db::DbConnection;
use dir_and_db_pool::db::get_db::get_db;
use fake::faker::creditcard::en::CreditCardNumber;
use fake::faker::internet::en::FreeEmail;
use fake::faker::internet::en::{Password, Username};
use fake::faker::name::en::Name;
use fake::faker::name::en::{FirstName, LastName};
use fake::{Dummy, Fake, Faker};
use r2d2_sqlite::rusqlite::params;

#[derive(Debug, Dummy)]
pub struct User {
    // #[dummy(faker = "1000..2000")]
    // id: Option<i32>,
    #[dummy(faker = "Username()")]
    username: String,
    #[dummy(faker = "Password(10..20)")]
    password: String,
    #[dummy(faker = "Password(10..20)")]
    salt: String,
    #[dummy(faker = "Name()")]
    name: Option<String>,
    #[dummy(faker = "FirstName()")]
    first_name: Option<String>,
    #[dummy(faker = "LastName()")]
    last_name: Option<String>,
    #[dummy(faker = "CreditCardNumber()")]
    personal_code: Option<String>,
    #[dummy(faker = "FreeEmail()")]
    email: Option<String>,
}

impl User {
    fn new_fake() -> Self {
        let user: User = Faker.fake();
        user
    }
}

/// Add user to db
pub fn add_user_to_db(db: &DbConnection, user: User, id: i64) -> Result<i64, TestSuiteError> {
    let username = format!("{}-{}", user.username, id);
    db.execute(
        "INSERT INTO user ( username, password, salt, name, first_name, last_name, personal_code, email) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            username,
            user.password,
            user.salt,
            user.name,
            user.first_name,
            user.last_name,
            user.personal_code,
            user.email,
        ],
    )?;

    let last_id = db.last_insert_rowid();
    Ok(last_id)
}

fn main() {
    let pool = get_db("centrale.db", "centrale").unwrap();
    let db = pool.get().expect("Couldn't get db connection from pool");
    // CREATE MILION ENTRIES
    for i in 0..1000000 {
        let user = User::new_fake();
        match add_user_to_db(&db, user, i) {
            Ok(id) => {
                println!("{}", id)
            }
            Err(err) => {
                eprintln!("err {}", err)
            }
        }
    }
}
