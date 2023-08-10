use std::path::PathBuf;

use crate::strings::{
    LENGTH_NOT_PROVIDED, MISSING_DEST_DIR, MISSING_POSITIONAL_ARGS_MSG, NO_ARGUMENTS_MSG,
    UNRECOGNIZED_ARGS_MSG, UNRECOGNIZED_OPTION_MSG,
};

#[derive(Debug)]
pub struct Options {
    pub quite: bool,
    pub in_place: bool,
    pub recursive: bool,
    pub print_help: bool,
    pub source_path: Option<PathBuf>,
    pub dest_path: Option<PathBuf>,
    pub sample_size: usize,
}

impl Options {
    fn default() -> Options {
        Options {
            quite: false,
            in_place: false,
            recursive: false,
            print_help: false,
            source_path: None,
            dest_path: None,
            sample_size: 33,
        }
    }
    pub fn parse(user_input: &Vec<String>) -> Result<Options, String> {
        // let user_input: Vec<String> = args.collect();

        match user_input.len() {
            1 => Err(NO_ARGUMENTS_MSG.to_string()),
            // one argument provided
            2 => {
                // check if that argument is the help flag
                match user_input[1].as_str() {
                    "-h" | "--help" => {
                        let mut opts = Options::default();
                        opts.print_help = true;
                        Ok(opts)
                    }
                    _ => Err(MISSING_POSITIONAL_ARGS_MSG.to_string()),
                }
            }
            // two or more arguments provided
            _ => {
                let mut opts = Options::default();

                // grab arguments which are flags, and which are positional
                let (flags, mut positional): (Vec<&String>, Vec<&String>) =
                    user_input.iter().partition(|item| item.starts_with('-'));

                for flag in flags {
                    match flag.as_str() {
                        "-h" | "--help" => opts.print_help = true,
                        "-i" | "--in-place" => opts.in_place = true,
                        "-r" | "--recursive" => opts.recursive = true,
                        "-q" | "--quite" => opts.quite = true,
                        "-l" | "--length" => {
                            let sample_size = user_input
                                .iter()
                                // find the position of the length flag
                                .position(|item| item == flag)
                                .and_then(|i| {
                                    // get the adjacent argument if it exists
                                    let adj_i = i + 1;
                                    let adj_arg = user_input.iter().nth(adj_i)?;
                                    Some((adj_i, adj_arg))
                                })
                                .and_then(|(adjacent_pos, adjacent_arg)| {
                                    // parse the adjacent argument into an int
                                    // will fail if the arg is not a number, a flag, etc
                                    match adjacent_arg.parse::<usize>() {
                                        Ok(size) if size > 0 => Some((adjacent_pos -1, size)),
                                        _ => None,
                                    }
                                });
                            match sample_size {
                                Some((index_of_length_arg, size)) => {
                                    opts.sample_size = size;
                                    // remove size argument from positional vector
                                    positional.remove(index_of_length_arg);
                                }
                                None => return Err(LENGTH_NOT_PROVIDED.to_string()),
                            }
                        }
                        _ => return Err(format!("{} {}", UNRECOGNIZED_OPTION_MSG, flag)),
                    };
                }

                match positional.len() {
                    // this program is always provided as an argument
                    0 | 1 => return Err(MISSING_POSITIONAL_ARGS_MSG.to_string()),
                    2 => {
                        if opts.in_place {
                            match PathBuf::from(positional[1]).canonicalize() {
                                Ok(src) => opts.source_path = Some(src),
                                Err(l) => return Err(l.to_string()),
                            }
                        } else {
                            return Err(MISSING_DEST_DIR.to_string());
                        }
                    }
                    3 => {
                        match PathBuf::from(positional[1]).canonicalize() {
                            Ok(src) => opts.source_path = Some(src),
                            Err(l) => return Err(l.to_string()),
                        }
                        match PathBuf::from(positional[2]).canonicalize() {
                            Ok(dest) => opts.dest_path = Some(dest),
                            Err(l) => return Err(l.to_string()),
                        }
                    }
                    _ => return Err(UNRECOGNIZED_ARGS_MSG.to_string()),
                }

                Ok(opts)
            }
        }
    }
}
