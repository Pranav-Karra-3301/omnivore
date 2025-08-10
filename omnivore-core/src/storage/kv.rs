use crate::{Error, Result};
use rocksdb::{Options, DB};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;

pub struct KvStore {
    db: DB,
}

impl KvStore {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);

        let db = DB::open(&opts, path)
            .map_err(|e| Error::Storage(format!("Failed to open database: {e}")))?;

        Ok(Self { db })
    }

    pub fn put<K, V>(&self, key: K, value: &V) -> Result<()>
    where
        K: AsRef<[u8]>,
        V: Serialize,
    {
        let serialized = serde_json::to_vec(value)?;
        self.db
            .put(key, serialized)
            .map_err(|e| Error::Storage(format!("Failed to put value: {e}")))
    }

    pub fn get<K, V>(&self, key: K) -> Result<Option<V>>
    where
        K: AsRef<[u8]>,
        V: DeserializeOwned,
    {
        match self.db.get(key) {
            Ok(Some(data)) => {
                let value = serde_json::from_slice(&data)?;
                Ok(Some(value))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(Error::Storage(format!("Failed to get value: {e}"))),
        }
    }

    pub fn delete<K>(&self, key: K) -> Result<()>
    where
        K: AsRef<[u8]>,
    {
        self.db
            .delete(key)
            .map_err(|e| Error::Storage(format!("Failed to delete value: {e}")))
    }
}
