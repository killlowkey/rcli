mod cli;
mod process;
mod utils;

pub use cli::{Base64Format, Base64Subcommand, Opts, Subcommand, TextSignFormat, TextSubcommand};
pub use process::{
    process_csv, process_decode, process_encode, process_gen_pass, process_text_sign,
};
pub use utils::get_reader;
