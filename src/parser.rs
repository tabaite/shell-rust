use std::str;

pub struct SplitArgsBase {
    backing_str: String,
}

pub struct SplitArgsIter {
    base: SplitArgsBase,
    current_index: usize,
    length: usize,
}

impl SplitArgsIter {
    pub fn new(string: &str) -> Self {
        Self {
            base: SplitArgsBase{
                backing_str: string.to_owned(),
            },
            current_index: 0,
            length: string.len(),
        }
    }
}

#[test]
pub fn test_split_args_whitespace() {
    let string = "cat bat   sat rat fat";
    let mut args = SplitArgsIter::new(string);

    assert_eq!("cat", args.next().unwrap());
    assert_eq!("bat", args.next().unwrap());
    assert_eq!("sat", args.next().unwrap());
    assert_eq!("rat", args.next().unwrap());
    assert_eq!("fat", args.next().unwrap());
}

#[test]
pub fn test_split_args_single_quotes() {
    let string = "'cat sat'   s 'bat''fat'";
    let mut args = SplitArgsIter::new(string);

    assert_eq!("cat sat", args.next().unwrap());
    assert_eq!("s", args.next().unwrap());
    assert_eq!("batfat", args.next().unwrap());
}

#[test]
pub fn test_split_args_double_quotes() {
    let string = "\"joker\"\"joRker\"    \"la' le' lu' le' lo'\"";
    let mut args = SplitArgsIter::new(string);

    assert_eq!("jokerjoRker", args.next().unwrap());
    assert_eq!("la' le' lu' le' lo'", args.next().unwrap());
}

#[test]
pub fn test_split_args_escape_space() {
    let string = "hey\\ \\ \\ \\ man";
    let mut args = SplitArgsIter::new(string);

    assert_eq!("hey    man", args.next().unwrap());
}

impl Iterator for SplitArgsIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index >= self.length {
            return None;
        }

        // Indicates the seperator.
        #[derive(PartialEq, Eq)]
        enum ParamType {
            Whitespace,
            SingleQuoted,
            DoubleQuoted,
        }

        /*
        we have total control over the string, so we can reorganize it to suit our needs
        when we submit the next.


        then: when we reach the end of our parameter, we set our starting position
        (for the next iteration) to the character right after the parameter.
        iterate over our parameter, and "bubble" all of the quotes to the end.
        then, return our clipped string.
        */

        let bytes= &self.base.backing_str.as_bytes()[self.current_index..];

        // first: move through whitespace until we find the start of the next iter.
        let find_result = bytes.iter().enumerate().find(|(_, c)| { **c != b' ' });

        let (first_position, _) = match find_result {
            Some((f, _)) => (f, 0),
            // the rest is nothing but whitespace
            None => return None
        };

        // this will contain the list of indices to remove
        // RELATIVE TO THE START OF THE PARAMETER
        let mut quotes_list = vec![];

        // if the first char is a quote, it'll override anyways
        let mut current_param_type = ParamType::Whitespace;
        // used for escape characters
        let mut ignore_next = false;

        // next: use a while loop to scan through
        // our parameter. if we are currently inside a quote (denoted by current_param_type),
        // ignore all whitespace.
        // consuming the first char is fine since we already check it.
        for i in first_position..bytes.len() {
            let current_char = bytes[i];
            match current_char {
                b'\\' if current_param_type == ParamType::Whitespace && !ignore_next => {
                    ignore_next = true;
                },
                // if we want to ignore the next character, then do nothing
                _ if ignore_next => ignore_next = false,
                // single quote start
                b'\'' if current_param_type == ParamType::Whitespace => {
                    quotes_list.push(i - first_position);
                    current_param_type = ParamType::SingleQuoted;
                },
                // single quote end
                b'\'' if current_param_type == ParamType::SingleQuoted => {
                    quotes_list.push(i - first_position);
                    current_param_type = ParamType::Whitespace;

                },
                // double quote start
                b'"' if current_param_type == ParamType::Whitespace => {
                    quotes_list.push(i - first_position);
                    current_param_type = ParamType::DoubleQuoted;

                },
                // double quote end
                b'"' if current_param_type == ParamType::DoubleQuoted => {
                    quotes_list.push(i - first_position);
                    current_param_type = ParamType::Whitespace;

                },
                // whitespace (in whitespace mode)
                b' ' if current_param_type == ParamType::Whitespace => {
                    self.current_index += i+1;
                    let param_bytes = &bytes[first_position..i];

                    let param_without_quotes: Vec<u8> = param_bytes
                        .iter()
                        .enumerate()
                        // We can binary search since we iterate on bytes in order, any insertions will also be in order
                        .filter(|(i, _)| { !quotes_list.binary_search(i).is_ok() })
                        .map(|(_, v)| { *v })
                        .collect();

                    let param = unsafe { std::str::from_utf8_unchecked(&param_without_quotes.as_slice()).to_owned() };

                    let result = Some(param);
                    return result;
                    // we need to organize and get rid of the stuff
                },
                // just a normal character
                _ => {}
            }
        }

        let param_bytes = &bytes[first_position..];
        self.current_index += self.length;

        let param_without_quotes: Vec<u8> = param_bytes
            .iter()
            .enumerate()
            // We can binary search since we iterate on bytes in order, any insertions will also be in order
            .filter(|(i, _)| { !quotes_list.binary_search(i).is_ok() })
            .map(|(_, v)| { *v })
            .collect();

        let param = unsafe { std::str::from_utf8_unchecked(&param_without_quotes.as_slice()).to_owned() };
        let result = Some(param);
        return result;
    }
}
