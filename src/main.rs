use rocksdb::{DB, Options};

// TODO: complete the normal test in rocsdb ability 
// the next thing is to test the substrate interface usage guidline 



// FIXME: should remove the db file dir and build in makefile 
// need more prefix for these db storage 
const DB_PATH: &'static str = "data";


fn aux_get_byte_slice<T: AsRef<[u8]>>(source: &'_ T) -> &'_[u8] {
    source.as_ref()
}

// simple test for rocksdb plain 
fn simple_db_works() { 
    let path = format!("{}/simple_db_works", DB_PATH);
    {
        let db = DB::open_default(path).unwrap();
         assert!(db.put(b"k1", b"v1").is_ok());
        let result = db.get(b"k1").unwrap().unwrap();
        assert_eq!(aux_get_byte_slice(&result), b"v1");
    }
}

fn write_batch_works() {
    let path = format!("{}/simple_db_works", DB_PATH);
    {
        let db = DB::open_default(path).unwrap();
        {
            let mut batch = rocksdb::WriteBatch::default();
            assert!(db.get(b"k1").unwrap().is_none());
            assert_eq!(batch.len(), 0);
            assert!(batch.is_empty());
            batch.put(b"k1", b"v1");
            batch.put(b"k2", b"v2");
            batch.put(b"k3", b"v3");
            assert_eq!(batch.len(), 3);
            assert!(!batch.is_empty());
            assert!(db.get(b"k1").unwrap().is_none());
            let p = db.write(batch);
            assert!(p.is_ok());
            let r: Result<Option<Vec<u8>>, rocksdb::Error> = db.get(b"k1");
            assert_eq!(r.unwrap().unwrap(), b"v1");
        }
        {
            let mut btach = rocksdb::WriteBatch::default();
            batch.delete(b"k1");
            assert_eq!(batch.len(), 1);
            let p = db.write(batch);
            assert!(p.is_ok());
            assert!(db.get(b"k1").unwrap().is_none());
        }
    }        
}


fn iterator_works() {
    let path = format!("{}/iterator_works", DB_PATH);
    {
        let data = [(b"k1", b"v1"), (b"k2", b"v2"), (b"k3", b"v3")];
        let db = DB::open_default(path).unwrap();
        for (k, v) in &data {
            assert!(db.put(k, v).is_ok());
        }
        let iter = db.iterator(rocksdb::IteratorMode::Start);
        for (idx, (k, v)) in iter.enumerate() {
            let (key, value) = data[idx];
            assert!((&key[..], &value[..]), (k.as_ref(), v.as_ref()));
        }
    }
}


fn snapshot_works() {
    let path = format!("{}/snapshot_works", DB_PATH);
    {
        let db = rocksdb::DB::open_default(path).unwrap();
        assert!(db.put(b"k1", b"v1").is_ok());
        let snap = db.snapshot();
        assert_eq!(snap.get(b"k1").unwrap().unwrap(), b"v1");
        assert!(db.put(b"k2", b"v2").is_ok());
        assert!(db.get("k2").unwrap().is_some());
        assert!(snap.get("k2").unwrap().is_none());

    }
}


// TODO: there need a feature for normal compile 
fn main() {
    simple_db_works();
    write_batch_works();
    iterator_works();
    snapshot_works();
}


// TODO: should add some SGX features test for Gramine

