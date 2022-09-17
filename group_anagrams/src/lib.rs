use std::{collections::HashMap, hash::Hash};

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
let mut groups: Vec<Vec<String>> = Vec::new();

    // iter through the words
    // check if the current word is an anagram of the first group
    // if yes insert it into that group
    // go to the next group
    // if any group was found, create a new group

    for word in strs {
        let mut pushed = false;

        for group in groups.iter_mut() {
            let first_word = group.first().unwrap();

            match is_anagram(first_word.clone(), word.clone()) {
                true => {
                    group.push(word.clone());
                    pushed = true;
                }
                false => continue
            };
        }

        if !pushed {
            groups.push(vec![word])
        }
    }

    groups
}

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
    use super::*;

    #[test]
    fn first_test() {
        let input = vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ];

        assert_eq!(
            group_anagrams(input),
            vec![vec!["bat"],vec!["nat","tan"],vec!["ate","eat","tea"]]
        );
    }
}