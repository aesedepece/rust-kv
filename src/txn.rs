use lmdb;
use lmdb::Transaction;

use error::Error;
use store::Bucket;
use cursor::{RoCursor, RwCursor};
use types::{Key, Value};

pub enum Txn<'env> {
    ReadOnly(lmdb::RoTransaction<'env>),
    ReadWrite(lmdb::RwTransaction<'env>)
}

impl <'env> Txn<'env> {
    pub fn read_only(t: lmdb::RoTransaction<'env>) -> Txn<'env> {
        Txn::ReadOnly(t)
    }

    pub fn read_write(t: lmdb::RwTransaction<'env>) -> Txn<'env> {
        Txn::ReadWrite(t)
    }

    pub fn commit(self) -> Result<(), Error> {
        match self {
            Txn::ReadOnly(txn) => Ok(txn.commit()?),
            Txn::ReadWrite(txn) => Ok(txn.commit()?)
        }
    }

    pub fn abort(self) {
        match self {
            Txn::ReadOnly(txn) => txn.abort(),
            Txn::ReadWrite(txn) => txn.abort()
        }
    }

    pub fn get<K: Key, V: Value<'env>>(&'env self, bucket: &Bucket, key: K) -> Result<V, Error> {
        match self {
            &Txn::ReadOnly(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key)?)),
            &Txn::ReadWrite(ref txn) => Ok(V::from_raw(txn.get(bucket.db(), &key)?))
        }
    }

    pub fn set<'a, K: Key, V: Value<'a>>(&mut self, bucket: &Bucket, key: K, val: V) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.put(bucket.db(), &key, &val, lmdb::WriteFlags::empty())?),
        }
    }

    pub fn del<K: Key>(&mut self, bucket: &Bucket, key: K) -> Result<(), Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(txn.del(bucket.db(), &key, None)?),
        }
    }

    pub fn read_cursor(&self, bucket: &Bucket) -> Result<RoCursor, Error> {
        match self {
            &Txn::ReadOnly(ref txn) => Ok(RoCursor(txn.open_ro_cursor(bucket.db())?)),
            &Txn::ReadWrite(ref txn) => Ok(RoCursor(txn.open_ro_cursor(bucket.db())?))
        }
    }

    pub fn write_cursor(&mut self, bucket: &Bucket) -> Result<RwCursor, Error> {
        match self {
            &mut Txn::ReadOnly(_) => Err(Error::ReadOnly),
            &mut Txn::ReadWrite(ref mut txn) => Ok(RwCursor(txn.open_rw_cursor(bucket.db())?))
        }
    }
}
