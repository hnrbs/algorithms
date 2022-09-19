use strings_encoder::*;

fn main() {
    let input = vec!["lint","code","love","you"];

    println!("{:?}", encode(input.clone()));
    dbg!(decode(encode(input)));
    panic!();
}