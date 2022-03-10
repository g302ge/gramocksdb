use rocksdb::{DB, Options};

// TODO: complete the normal test in rocsdb ability 
// the next thing is to test the substrate interface usage guidline 



// FIXME: should remove the db file dir and build in makefile 
// need more prefix for these db storage 
const DB_PATH: &'static str = "test.db";


fn aux_get_byte_slice<T: AsRef<[u8]>>(source: &'_ T) -> &'_[u8] {
    source.as_ref()
}

// simple test for rocksdb plain 
fn normal_db_works() { 
    {
        let db = DB::open_default(DB_PATH).unwrap();
         assert!(db.put(b"k1", b"v1").is_ok());
        let result = db.get(b"k1").unwrap().unwrap();
        assert_eq!(aux_get_byte_slice(&result), b"v1");
    }
}



fn main() {

    let path = "./test_dir";
    let db = DB::open_default(path).unwrap();
    db.put(b"key", b"value").unwrap();


    DB::destroy(&Options::default(), path);
}

