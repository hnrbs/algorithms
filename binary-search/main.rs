use std::io;

fn main() {
    let sorted_array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let search = get_search();

    let position = find_in(sorted_array, search);

    println!("Found the value {} on position {}.", search, position);
}

fn get_search() -> i32 { 
    loop {    
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("something wrong happened");

        match input.trim().parse() {
            Ok(number) => break number,
            Err(_)     => println!("You should type a number"),
        }
    }
}

fn find_in(sorted_array: [i32; 10], search: i32) -> u32 {
    let mut high = 10;
    let mut low  = 0;
    let mut middle = 0;

    loop {
        if sorted_array[middle as usize] == search {
            break middle
        }

        if sorted_array[middle as usize] > search {
            low = middle + 1; 
        }

        if sorted_array[middle as usize] < search {
            high = middle - 1;
        }
    }
}

