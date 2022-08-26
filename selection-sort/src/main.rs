fn main() {
}

#[allow(dead_code)]
fn select_sort(unsorted_list: &Vec<i32>) -> Vec<i32> {
    let mut unsorted_list = unsorted_list.clone();
    let mut sorted_list = Vec::new();

    for _item in 0..unsorted_list.len() {
        let smallest = find_smallest(&unsorted_list);
        let removed = unsorted_list.remove(smallest);
        sorted_list.push(removed);
    }

    sorted_list
}

#[allow(dead_code)]
fn find_smallest(list: &Vec<i32>) -> usize {
    let mut smallest = list.first().expect("Wtf, you sent me an empty array!");
    let mut smallest_index = 0usize;

    for (index, value) in list.iter().enumerate() {
        if value < smallest {
            smallest = value;
            smallest_index = index;
        };
    };

    smallest_index
}

#[cfg(test)]
mod tests {
    use crate::select_sort;

    #[test]
    fn it_sorts_a_list()
    {
        let unsorted_list = vec![5, 3, 4, 1, 2];

        assert_eq!(
            select_sort(&unsorted_list),
            vec![1, 2, 3, 4, 5]
        );
    }
}