use std::cmp::PartialEq;
use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
struct HashMap<T, R> {
    map: Vec<Map<T, R>>,
    size: usize,
}

#[derive(Default, Clone, Debug)]
struct Map<T, R> {
    key: T,
    value: R,
    taken: bool,
}

trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for String {
    fn hash(&self) -> usize {
        let mut result: usize = 5038;

        for c in self.bytes() {
            result = ((result << 5).wrapping_add(result)).wrapping_add(c.into());
        }

        result
    }
}

impl<
        T: PartialEq + Clone + Default + Debug + Display + Hashable,
        R: Clone + Default + Debug + Display + Hashable,
    > HashMap<T, R>
{
    fn new() -> Self {
        const INITIAL_CAPACITY: usize = 11;
        Self {
            map: vec![Map::default(); INITIAL_CAPACITY],
            size: INITIAL_CAPACITY,
        }
    }

    fn extend(&mut self) {
        let new_capacity: usize = self.map.len() * 2 + 1;
        let mut new_map = Self {
            map: vec![Map::default(); new_capacity],
            size: new_capacity,
        };
        for element in self.map.iter() {
            let index_to_insert = element.key.hash() % new_map.size;
            new_map.map.insert(index_to_insert, element.clone());
        }

        *self = new_map;
    }

    fn insert(&mut self, key: T, value: R) {
        let mut hash_index = key.hash() % self.size;
        while self.map[hash_index].taken {
            if self.map[hash_index].key == key {
                self.map[hash_index].value = value;
                return;
            }

            hash_index += 1;
            if hash_index >= self.size - 1 {
                self.extend();
            }
        }

        self.map[hash_index].value = value;
        self.map[hash_index].key = key;
        self.map[hash_index].taken = true;
    }
    fn get(&self, key: &T) -> Option<&R> {
        let mut hash_index = key.hash() % self.size;

        println!("size: {}", self.size);
        println!("index: {}", hash_index);
        println!("value: {}", self.map[hash_index].key);
        while self.map[hash_index].key != *key && hash_index <= self.size {
            println!("Loop: {}", self.map[hash_index].key);

            if !self.map[hash_index].taken {
                return None;
            }

            hash_index += 1;
        }

        if hash_index <= self.size && self.map[hash_index].key != *key {
            return None;
        }
        println!("before loop index: {}", hash_index);
        Some(&self.map[hash_index].value)
    }

    fn debug(&self) {
        println!("debug size: {}", self.size);

        for element in &self.map {
            if element.taken {
                println!(
                    "{} --> {}  index ---> {}",
                    element.key,
                    element.value,
                    element.key.hash() % self.size
                );
            } else {
                println!("X");
            }
        }
    }
}

fn main() {
    let mut my_hash = HashMap::<String, String>::new();

    my_hash.insert("to_change_value".to_string(), "First Value".to_string());
    my_hash.insert("GetMeeBebe".to_string(), "Valor sou eu".to_string());
    for i in 0..50 {
        let key = format!("Nome{}", i);
        my_hash.insert(key.to_string(), "Gabriel".to_string());
    }

    my_hash.insert("to_change_value".to_string(), "Last Value".to_string());
    my_hash.debug();

    println!("--------------------------");

    let getReturn = my_hash.get(&"GetMeeBebe".to_string());

    match getReturn {
        Some(value) => println!("Get GetMeeBebe value {}", value),
        None => println!("Not found!!"),
    }
}
