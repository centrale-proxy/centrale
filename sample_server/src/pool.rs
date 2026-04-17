pub mod add;
pub mod get;

use dir_and_db_pool::db::DbPool;
use std::collections::HashMap;

pub struct DbPoolRegistry {
    pub pools: HashMap<String, DbPool>,
}

impl DbPoolRegistry {
    pub fn get(&self, key: &str) -> Option<&DbPool> {
        self.pools.get(key)
    }
}
