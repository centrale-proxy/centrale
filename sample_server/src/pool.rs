pub mod add;
pub mod get;

use dir_and_db_pool::db::DbBool;
use std::collections::HashMap;

pub struct DbPoolRegistry {
    pub pools: HashMap<String, DbBool>,
}

impl DbPoolRegistry {
    pub fn get(&self, key: &str) -> Option<&DbBool> {
        self.pools.get(key)
    }
}
