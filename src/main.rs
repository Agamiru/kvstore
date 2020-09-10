use kvstore::KvStore;





fn main() {
    let mut store = KvStore::new();

    store.set("cat", "meow");       // output: Record successfully set
    store.set("dog", "bark");       // output: Record successfully set
    store.set("horse", "neigh");    // output: Record successfully set

    let dog_sound = store.get("dog");
    let cat_sound = store.get("cat");
    store.remove("horse");          // output: Item with key 'horse' and value 'neigh' successfully removed

    assert_eq!(Some("bark".to_string()), dog_sound);
    assert_eq!(Some("meow".to_string()), cat_sound);
    assert_eq!(None, store.get("horse"));       // output: Item doesn't exist in store

}
