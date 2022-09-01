use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;

#[allow(unused)]
#[derive(Debug)]
struct Slot<K: Hash + Eq, V> {
    data: Vec<(K, V)>,
}

impl<K: Hash + Eq, V> Slot<K, V> {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn from(key: K, value: V) -> Self {
        Self {
            data: Vec::from([(key, value)]),
        }
    }

    pub fn get_index(&self, key: &K) -> Result<usize, ()> {
        let result = self
            .data
            .iter()
            .enumerate()
            .find(|(_, (slot_key, _))| slot_key == key);

        match result {
            Some((index, _)) => Ok(index),
            None => Err(()),
        }
    }
}

#[derive(Default, Debug)]
#[allow(unused)]
pub struct HashMap<K: Hash + Eq, V> {
    slots: Vec<Slot<K, V>>,
    items: u64,
}

#[derive(Debug)]
pub enum Error {
    EmptyMap,
    DuplicatedKey,
    NonexistentKey,
}

impl<K: Hash + Eq, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            items: 0,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.resize();

        let index = self.get_index(&key, self.slots.len());
        let slot = &mut self.slots[index];

        if let Some((_, slot_value)) = slot.data.iter_mut().find(|(slot_key, _)| slot_key == &key) {
            return Some(mem::replace(slot_value, value));
        }

        slot.data.push((key, value));
        self.items += 1;
        None
    }

    pub fn get(&mut self, key: K) -> Option<&V> {
        let index = self.get_index(&key, self.slots.len());

        let slot = match self.slots.get(index) {
            Some(slot) => slot,
            None => return None,
        };

        let value = slot.data.iter().find(|(slot_key, _)| &key == slot_key);

        match value {
            Some((_, value)) => Some(value),
            None => None,
        }
    }

    pub fn remove(&mut self, key: K) -> Result<V, Error> {
        let index = self.get_index(&key, self.slots.len());

        let slot = match self.slots.get_mut(index) {
            Some(slot) => slot,
            None => return Err(Error::NonexistentKey),
        };

        let index = match slot.get_index(&key) {
            Ok(index) => index,
            Err(_) => return Err(Error::NonexistentKey),
        };

        let (_, removed_value) = slot.data.swap_remove(index);

        self.items -= 1;
        Ok(removed_value)
    }

    pub fn len(&self) -> usize {
        self.items as usize
    }

    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    fn get_index(&self, key: &K, len: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % len as u64) as usize
    }

    fn resize(&mut self) {
        if !self.slots.is_empty() || self.items < 7 * (self.slots.len() as u64) / 10 {
            return;
        }

        let size = match self.slots.len() {
            0 => 1,
            n => n * 2,
        };

        let mut resized_slots = Vec::with_capacity(size);
        resized_slots.extend((0..size).map(|_| Slot::new()));

        for (key, value) in self.slots
            .iter_mut()
            .flat_map(|slot| slot.data.drain(..))
        {
            let mut hasher = DefaultHasher::default();
            key.hash(&mut hasher);
            let index = (hasher.finish() % resized_slots.len() as u64) as usize;

            resized_slots[index].data.push((key, value));
        }

        let _ = mem::replace(&mut self.slots, resized_slots);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get_map_key_index() {
        let mut map = HashMap::new();

        let _ = map.insert("foo", "bar");

        assert_eq!(map.get_index(&"foo", map.slots.len()), 0)
    }
    #[test]
    fn can_insert_values() {
        let mut map = HashMap::new();
        map.insert("foo", "bar");
    }

    #[test]
    fn can_get_items() {
        let mut map = HashMap::new();
        map.insert("foo", "bar");
        assert_eq!(map.get("foo"), Some(&"bar"));
    }

    #[test]
    fn can_remove_item() {
        let mut map = HashMap::new();
        map.insert("foo", "bar");

        let result = map.remove("foo");

        assert!(result.is_ok());
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn can_get_the_map_len() {
        let mut map = HashMap::new();
        map.insert("name", "henri");
        assert_eq!(map.len(), 1);
    }
}
