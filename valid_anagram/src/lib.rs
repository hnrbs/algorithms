use std::{collections::HashMap, iter::Zip};

pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let zip = s.chars().into_iter().zip(t.chars().into_iter());

    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();

    for (s, t) in zip {
        match map_s.contains_key(&s) {
            true  => {
                let current_value = map_s.get(&s).unwrap();

                map_s.insert(s, current_value + 1);
            },
            false => {
                map_s.insert(s, 1);
            }
        }

        match map_t.contains_key(&t) {
            true  => {
                let current_value = map_t.get(&t).unwrap();

                map_t.insert(t, current_value + 1);
            },
            false => {
                map_t.insert(t, 1);
            }
        }
    };

    for (key, value) in map_s.iter() {
        match map_t.get(key) {
            Some(t) => {
                if t != value {
                    return false;
                }
            },
            None => {
                return false
            }
        }
    };

    true
}

#[cfg(test)]
mod test {
    use crate::is_anagram;

    #[test]
    fn pog_should_return_true() {
        let s = String::from("anagram");
        let t = String::from("nagaram");

        assert!(is_anagram(s, t));
    }

    #[test]
    fn should_return_false() {
        let s = String::from("abc");
        let t = String::from("abcde");

        assert!(!is_anagram(s, t));
    }
}
