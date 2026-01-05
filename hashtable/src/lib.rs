use std::hash::{DefaultHasher, Hash, Hasher};

const INITIAL_CAPACITY: usize = 16;

pub struct HashTable<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K: Clone + Hash + Eq, V: Clone> HashTable<K, V> {
    pub fn new() -> Self {
        Self {
            buckets: vec![vec![]; INITIAL_CAPACITY],
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash_value: u64 = hasher.finish();
        hash_value as usize % self.buckets.len()
    }

    pub fn put(&mut self, key: K, value: V) {
        let hash = self.hash(&key);
        match self.buckets[hash].iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => {
                *v = value;
            }
            None => {
                self.buckets[hash].push((key, value));
                self.len += 1;
            }
        }
        if self.len as f32 > (self.buckets.len() as f32) * 0.75 {
            self.resize();
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(key);
        self.buckets[hash]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }

    pub fn delete(&mut self, key: &K) -> Option<V> {
        let hash = self.hash(key);
        let result = self.buckets[hash]
            .iter()
            .position(|(k, _)| k == key)
            .map(|pos| self.buckets[hash].remove(pos).1);
        if result.is_some() {
            self.len -= 1;
        }
        result
    }

    fn resize(&mut self) {
        let new_capacity = self.buckets.len() * 2;
        let old_buckets = std::mem::replace(&mut self.buckets, vec![vec![]; new_capacity]);
        self.len = 0;
        old_buckets
            .into_iter()
            .flatten()
            .for_each(|(k, v)| self.put(k, v));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_hashtable() {
        let _: HashTable<String, i32> = HashTable::new();
    }

    #[test]
    fn hash_returns_valid_index() {
        let ht: HashTable<String, i32> = HashTable::new();
        let index = ht.hash(&"hello".to_string());
        assert!(index < 16); // INITIAL_CAPACITY 범위 내
    }

    #[test]
    fn put_single_item() {
        let mut ht = HashTable::new();
        ht.put("name".to_string(), "alice".to_string());
        // 에러 없이 저장되면 성공
    }

    #[test]
    fn put_overwrites_existing_key() {
        let mut ht = HashTable::new();
        ht.put("name".to_string(), "alice".to_string());
        ht.put("name".to_string(), "bob".to_string());
        // 같은 키에 덮어쓰기 가능해야 함
    }

    #[test]
    fn get_existing_key() {
        let mut ht = HashTable::new();
        ht.put("name".to_string(), "alice".to_string());
        assert_eq!(ht.get(&"name".to_string()), Some(&"alice".to_string()));
    }

    #[test]
    fn get_nonexistent_key() {
        let ht: HashTable<String, String> = HashTable::new();
        assert_eq!(ht.get(&"name".to_string()), None);
    }

    #[test]
    fn handle_collision() {
        // 작은 capacity로 충돌 유도
        let mut ht: HashTable<i32, &str> = HashTable::new();
        // 0과 16은 capacity=16일 때 같은 인덱스
        ht.put(0, "zero");
        ht.put(16, "sixteen");
        assert_eq!(ht.get(&0), Some(&"zero"));
        assert_eq!(ht.get(&16), Some(&"sixteen"));
    }

    #[test]
    fn delete_existing_key() {
        let mut ht = HashTable::new();
        ht.put("name".to_string(), "alice".to_string());
        let deleted = ht.delete(&"name".to_string());
        assert_eq!(deleted, Some("alice".to_string()));
        assert_eq!(ht.get(&"name".to_string()), None);
    }

    #[test]
    fn delete_nonexistent_key() {
        let mut ht: HashTable<String, String> = HashTable::new();
        let deleted = ht.delete(&"name".to_string());
        assert_eq!(deleted, None);
    }

    #[test]
    fn resize_on_high_load_factor() {
        let mut ht = HashTable::new();
        for i in 0..100 {
            ht.put(i, i * 10);
        }
        // 모든 항목 조회 가능해야 함
        for i in 0..100 {
            assert_eq!(ht.get(&i), Some(&(i * 10)));
        }
    }

    #[test]
    fn len_correct_after_resize() {
        let mut ht = HashTable::new();
        for i in 0..20 {
            ht.put(i, i);
        }
        assert_eq!(ht.len(), 20); // 이전엔 40이 됐을 것
    }

    #[test]
    fn len_tracks_item_count() {
        let mut ht = HashTable::new();
        assert_eq!(ht.len(), 0);
        ht.put("a".to_string(), 1);
        assert_eq!(ht.len(), 1);
        ht.put("a".to_string(), 2); // 덮어쓰기, len 변화 없음
        assert_eq!(ht.len(), 1);
        ht.delete(&"a".to_string());
        assert_eq!(ht.len(), 0);
    }
}
