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

        // TODO: handle the case where it's just one big parameter

        // if the first char is a quote, it'll override anyways
        let mut current_param_type = ParamType::Whitespace;
        // next: use a while loop to scan through
        // our parameter. if we are currently inside a quote (denoted by current_param_type),
        // ignore all whitespace.
        // consuming the first char is fine since we already check it.
        for i in first_position..bytes.len() {
            let current_char = bytes[i];
            match current_char {
                // single quote start
                b'\'' if current_param_type == ParamType::Whitespace => {
                    current_param_type = ParamType::SingleQuoted;
                },
                // single quote end
                b'\'' if current_param_type == ParamType::SingleQuoted => {
                    current_param_type = ParamType::Whitespace;

                },
                // double quote start
                b'"' if current_param_type == ParamType::Whitespace => {
                    current_param_type = ParamType::DoubleQuoted;

                },
                // double quote end
                b'"' if current_param_type == ParamType::DoubleQuoted => {
                    current_param_type = ParamType::Whitespace;

                },
                // whitespace (in whitespace mode)
                b' ' if current_param_type == ParamType::Whitespace => {
                    self.current_index += i+1;
                    let param_bytes = &bytes[first_position..i];
                    let param = unsafe { std::str::from_utf8_unchecked(&param_bytes).to_owned() }.replace(['"', '\''], "");

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

        let result = Some( unsafe { std::str::from_utf8_unchecked(&param_bytes).to_owned() }.replace(['"', '\''], "") );
        return result;
    }
}
