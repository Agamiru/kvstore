use std::collections::HashMap;

/// `kvstore` is a simple in-memory key-value store implementation in Rust.
/// 
/// ## Expectation
/// It is expected morphose into a command line application with 
/// persistent data storage and proper logging. 
/// 
/// ## Example
/// ```
/// use kvstore::KvStore;
/// 
/// let mut store = KvStore::new();
/// store.set("cat", "meow");
/// store.set("dog", "bark");
/// store.set("horse", "neigh");
/// 
/// let dog_sound = store.get("dog");
/// let cat_sound = store.get("cat");
/// store.remove("horse");
/// 
/// assert_eq!(Some("bark".to_string()), dog_sound);
/// assert_eq!(Some("meow".to_string()), cat_sound);
/// assert_eq!(None, store.get("horse"));
/// ```

#[derive(Debug)]
pub struct KvStore {
    map: HashMap<String, String>,
}


// Store methods
impl KvStore {
    
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> Option<String> {
        let updated_val = self.map.insert(key.to_owned(), value.to_owned());
        match updated_val {
            Some(_) => {
                let val = updated_val.clone().unwrap();
                println!("Value '{}' was updated to {}", value, val)
            },
            None => println!("Record successfully set"),
        };

        updated_val
    }

    pub fn get(&self, key: &str) -> Option<String> {
        let val = self.map.get(key);
        if val.is_none() {
            println!("Item doesn't exist in store");
            return None;
        }
        
        // Option<&String> returned by get method, cloned converts inner value to owned
        val.cloned()

    }
    pub fn remove(&mut self, key: &str) -> Option<String> {
        let removed_val = self.map.remove(key);
        match removed_val {
            Some(_) => { 
                let val = removed_val.clone().unwrap();
                println!("Item with key '{}' and value '{}' successfully removed", key, val);
            },
            None => println!("No such item '{}' exists", key),
        };
        
        removed_val
    }

    pub fn iter(&self) -> KvStoreIter {
        let mut vec_list = Vec::new();
        for (k, v) in &self.map{
            let (key, val) = (k, v);    // k, v are string references
            vec_list.push(format!("k: {}, v: {}", key, val));
        };
        
        KvStoreIter {
            index: 0,
            vec: vec_list
        }
    }

}

// Store traits
impl Clone for KvStore {
    fn clone(&self) -> KvStore {
        let cloned_map = self.map.clone();
        KvStore { map: cloned_map }
    }
}

impl PartialEq for KvStore {
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}

// Store to_iter method return object
pub struct KvStoreIter {
    index: usize,
    vec: Vec<String>,
}

impl Iterator for KvStoreIter {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let max_length = self.vec.len();

        if self.index < max_length {
            let value = self.vec[self.index].clone();
            self.index += 1;
            return Some(value)
        }
        None
    }
}


// Tests
#[cfg(test)]

mod test {
    use super::*;

    fn setup() -> KvStore {
        KvStore::new()
    }
    
    fn setup_with_data() -> KvStore {
        let mut maps = HashMap::new();
        maps.insert("cat".to_owned(), "meow".to_owned());
        maps.insert("dog".to_owned(), "bark".to_owned());
        maps.insert("horse".to_owned(), "neigh".to_owned());
    
        KvStore { map: maps }
    }

    #[test]
    fn test_store_set() {
        let mut store = setup();
        store.set("cat", "meow");
        store.set("dog", "bark");
        store.set("horse", "neigh");

        assert_eq!(store.map.len(), 3);
    }

    #[test]
    fn test_store_set_with_existing_key() {
        let mut store = setup();
        store.set("dog", "bark");
        store.set("dog", "woof");
        let no_value = store.set("seal", "yelp");

        assert_eq!(store.map.get("dog").cloned(), Some("woof".to_owned()));
        assert_eq!(None, no_value)
    }

    #[test]
    fn test_store_get() {
        let store = setup_with_data();
        let val = store.get("cat");
        let val_2 = store.get("dog");
        let val_3 = store.get("rabbit");
      
        assert_eq!(Some("meow".to_owned()), val);
        assert_eq!(Some("bark".to_owned()), val_2);
        assert_eq!(None, val_3);
    }

    #[test]
    fn test_store_remove() {
        let mut store = setup_with_data();
        store.remove("cat");
        store.remove("dog");
        assert_eq!(1, store.map.len());
    }

    #[test]
    fn test_store_remove_with_wrong_key() {
        let mut store = setup_with_data();
        store.remove("cat");
        let removed_value = store.remove("fish");

        assert_eq!(2, store.map.len());
        assert_eq!(None, removed_value)
    }

    #[test]
    fn test_store_iter() {
        let store = setup_with_data();
        let mut iter = store.iter();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.index, 3)
    }

    #[test]
    fn test_store_clone_and_eq_trait() {
        let store = setup_with_data();
        let new_store = store.clone();

        assert_eq!(store, new_store);
        assert_eq!(store.map, new_store.map)
    }

}






