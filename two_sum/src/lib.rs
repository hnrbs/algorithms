use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        match map.contains_key(&(target - num)) {
            true => {
                let index = map.get(&(target - num)).unwrap();
                return vec![*index, i as i32];
            }
            false => {
                map.insert(num, i as i32);
            }
        }
    };

    panic!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn target_nine() {
        let nums = vec![2,7,11,15];

        assert_eq!(two_sum(nums, 9), vec![0, 1]);
    }

    #[test]
    fn target_six() {
        let nums = vec![3, 2, 4];

        assert_eq!(two_sum(nums, 6), vec![1, 2]);
    }
}