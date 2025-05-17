use std::str;

pub struct SplitArgsIter<'a> {
    backing_str: &'a str,
    current_index: usize,
    length: usize,
}

impl<'a> SplitArgsIter<'a> {
    pub fn new(string: &'a str) -> Self {
        Self {
            backing_str: string,
            current_index: 0,
            length: string.len(),
        }
    }
}

#[test]
pub fn test_split_args_whitespace() {
    let string = "cat bat sat rat fat";
    let mut args = SplitArgsIter::new(string);

    assert_eq!("cat", args.next().unwrap());
    assert_eq!("bat", args.next().unwrap());
    assert_eq!("sat", args.next().unwrap());
    assert_eq!("rat", args.next().unwrap());
    assert_eq!("fat", args.next().unwrap());
}

#[test]
pub fn test_split_args_single_quotes() {
    let string = "'cat sat' 'bat' 'fat'";
    let mut args = SplitArgsIter::new(string);

    assert_eq!("cat sat", args.next().unwrap());
    assert_eq!("bat", args.next().unwrap());
    assert_eq!("fat", args.next().unwrap());
}

impl<'a> Iterator for SplitArgsIter<'a> {
    type Item = &'a str;

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

        let mut current_param_type = ParamType::Whitespace;

        let bytes = self.backing_str.as_bytes();
        for i in self.current_index..self.length {
            let current_char = bytes[i];
            match current_char {
                //                                                     we don't want a bunch of blank params
                b' ' if current_param_type == ParamType::Whitespace && self.current_index != i => {
                    let result = Some(unsafe { str::from_utf8_unchecked(&bytes[self.current_index..i]) });
                    // We want to start at the space AFTER we encounter the space.
                    // If this causes current index to be greater than the length,
                    // it's still safe.
                    self.current_index = i+1;
                    return result;
                },
                b'\'' if current_param_type == ParamType::Whitespace => {
                    current_param_type = ParamType::SingleQuoted;
                    self.current_index = i+1;
                },
                b'\'' if current_param_type == ParamType::SingleQuoted => {
                    let result = Some(unsafe { str::from_utf8_unchecked(&bytes[self.current_index..i]) });
                    // We want to start at the space AFTER we encounter the space.
                    // If this causes current index to be greater than the length,
                    // it's still safe.
                    self.current_index = i+1;
                    return result;
                }
                _ => {}
            }
        }
        // We've reached the end
        // what we have currently is what we will return
        let result = Some(unsafe { str::from_utf8_unchecked(&bytes[self.current_index..self.length]) });
        self.current_index = self.length;
        result
    }
}
