use rocksdb::DB;

pub(crate) struct Database {
    db: DB,
}

impl Database {
    pub(crate) fn new(path: String) -> anyhow::Result<Self> {
        let db = DB::open_default(path.clone())?;

        Ok(Self { db })
    }

    pub(crate) fn put(&self, key: &[u8], value: &[u8]) {
        let _ = self.db.put(key, value);
    }

    pub(crate) fn get(&self, key: &[u8]) -> anyhow::Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?)
    }
}
