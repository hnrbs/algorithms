
fn main() {
    println!("Quick sort!");
}

#[allow(dead_code)]
fn quick_sort<T>(unsorted_list: &Vec<T>) -> Vec<T>
    where T: Ord + Copy
{
    if unsorted_list.len() < 2 {
        return unsorted_list.clone();
    }

    let pivot = unsorted_list.first().unwrap();
    let mut less = Vec::new();
    let mut greater = Vec::new();

    for i in &unsorted_list[1..] {
        if *i <= *pivot {
            less.push(*i);
        } else {
            greater.push(*i);
        }
    }

    vec![quick_sort(&less), vec![*pivot], quick_sort(&greater)]
        .into_iter()
        .flatten()
        .collect()
}

#[cfg(test)]
mod test {
    use super::quick_sort;

    #[test]
    fn it_sorts_a_list() {
        assert_eq!(
            quick_sort(&vec![5, 2, 3, 1, 4]),
            vec![1, 2, 3, 4, 5]
        );

        assert_eq!(
            quick_sort(&vec![2, 1]),
            vec![1, 2]
        );
    }
}
