use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(arg_required_else_help = true)]
    Encode {
        file: String,
        chunk: String,
        message: String,
        output_file: Option<String>
    },

    #[command(arg_required_else_help = true)]
    Decode {
        file: String,
        chunk: String
    },

    #[command(arg_required_else_help = true)]
    Remove {
        file: String,
        chunk: String
    },

    #[command(arg_required_else_help = true)]
    Print {
        file: String
    }

}
