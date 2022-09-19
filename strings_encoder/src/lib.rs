use std::arch::x86_64::_MM_FROUND_CUR_DIRECTION;

// time complexity = O(n²)
// data complexity = O(n+n) = O(n)
pub fn encode(input: Vec<&str>) -> String {
    let mut output = String::new();

    for string in input  {
        let mut prepared_word = String::new();

        for char in string.chars() {
            if char == '#' {
                prepared_word.push_str("\\#");

                continue;
            }

            prepared_word.push(char)
        }

        output.push_str(
            format!("{}#", prepared_word).as_str()
        );
    }

    output
}

pub fn decode(input: String) -> Vec<String> {
    Decoder::decode(input)
}

#[derive(Debug)]
struct Decoder {
    input: String,
    current_index: u64
}

impl Decoder {
    pub fn decode(input: String) -> Vec<String> {
        let mut decoder = Self {
            input,
            current_index: 0
        };

        decoder.run()
    }

    // O(n²)
    fn run(&mut self) -> Vec<String> {
        let mut output = Vec::new();

        while (self.current_index as usize) < self.input.len() {
            let current_range = String::from(&self.input[
                (self.current_index) as usize..
            ]);

            let mut word = String::new();

            for char in current_range.chars() {
                match char {
                    '#' => {
                        self.current_index += 1;
                        break;
                    },
                    char => {
                        self.current_index += 1;
                        word.push(char);
                    },
                }
            }

            output.push(word);
        }

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_input() {
        let input = vec!["lint","code","love","you"];

        let encoded = encode(input.clone());
        assert_eq!(encoded, "lint#code#love#you#");

        assert_eq!(decode(encoded), input);
    }

    #[test]
    fn input_with_hashtag() {
        let input = vec!["lint", "code#", "love", "you"];

        let encoded = encode(input.clone());
        assert_eq!(encoded, "lint#code\\##love#you#");

        assert_eq!(decode(encoded), input);
    }
}