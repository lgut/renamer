
pub const HELP_MSG: &'static str = 
"renamer: [options] SOURCE_DIR DEST_DIR

Renames files by prepending a random string of characters to it
    
options:
    -h, --help                      Print help
    -l NUMBER, --length NUMBER      Specify the length of the random character string. Can not be 0 (Default: 33)
    -r, --recursive                 Rename files recursively. Directory structure is preserved in the DEST_DIR directory
    -i, --in-place                  Modify the files in place. DEST_DIR is ignored
    -q, --quite                     Silence output to console";

pub const NO_ARGUMENTS_MSG: &'static str = "No arguments were provided.";

pub const MISSING_POSITIONAL_ARGS_MSG: &'static str = "SOURCE_DIR and DEST_DIR arguments are required.";

pub const MISSING_DEST_DIR: &'static str = "DEST_DIR not specified.";

pub const UNRECOGNIZED_OPTION_MSG: &'static str = "Unrecognized Option:";
pub const UNRECOGNIZED_ARGS_MSG: &'static str = "Unknown position arguments were provided";
pub const LENGTH_NOT_PROVIDED: &'static str = "-l or --length option was used but length was not specified";
