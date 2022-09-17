use std::collections::HashMap;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashMap::new();

    for num in &nums {
        map.insert(num, ());
    }

    map.len() != nums.len()
}

#[cfg(test)]
mod test {
    use crate::contains_duplicate;

    #[test]
    fn it_contains_duplicated_numbers() {
        let numbers = vec![1,1,1,3,3,4,3,2,4,2];

        assert!(contains_duplicate(numbers));
    }
}
