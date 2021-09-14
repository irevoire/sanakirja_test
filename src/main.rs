use rstar::RTree;
use sanakirja::*;

fn main() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("db");

    let mut rtree = RTree::new();
    rtree.insert([0.1, 0.0f32]);
    rtree.insert([0.2, 0.1]);
    rtree.insert([0.3, 0.0]);

    let env = Env::new(&path, 1 << 20, 2).unwrap();
    let mut txn = Env::mut_txn_begin(&env).unwrap();
    let mut db = btree::create_db::<_, u8, [u8]>(&mut txn).unwrap();
    let array: [u8; 452] = unsafe { std::mem::transmute(rtree) };
    btree::put(&mut txn, &mut db, &1, &array);
    let root_db = 0;
    txn.set_root(root_db, db.db);
    txn.commit().unwrap();
    let txn = Env::txn_begin(&env).unwrap();
    let db: btree::Db<u64, u64> = txn.root_db(root_db).unwrap();
    assert_eq!(
        btree::get(&txn, &db, &50_000, None).unwrap(),
        Some((&50_000, &(50_000 * 50_000)))
    );
    for entry in btree::iter(&txn, &db, None).unwrap() {
        let (k, v) = entry.unwrap();
        assert_eq!(*k * *k, *v)
    }
}
