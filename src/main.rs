mod formatter;
mod parser;
mod validator;

// my local imports
use formatter::generate_latex_verbatim;
use parser::extract_highlighted_pieces;
use validator::{is_input_ok, is_output_ok};

// external imports
use clap::Parser;
use std::path::Path;

#[derive(Parser)]
#[command(
    author = "Tomáš Lebeda <tom.lebeda@gmail.com>",
    version = "1.0.0",
    about = "Generate LaTeX for highlighted code listings with the power of TreeSitter."
)]
pub struct CliArgs {
    /// Input file with the code that should be highlighted.
    #[arg(short, long)]
    pub input: Option<String>,

    /// Output file where generated LaTeX code will be stored.
    #[arg(short, long)]
    pub output: Option<String>,

    /// String that marks start of evaluation inside LaTeX's verbatim environment.
    #[arg(long, default_value_t = String::from("<@"))]
    pub escape_start: String,

    /// String that marks end of evaluation inside LaTeX's verbatim environment.
    #[arg(long, default_value_t = String::from("@>"))]
    pub escape_end: String,

    /// If enabled, the output will not be wrapped by verbatim environment, it will be only the "raw insides".
    #[arg(short, long)]
    pub raw: bool,

    /// Generate necessary directories/files on disk and overwrite anything that stands in the way.
    #[arg(short, long)]
    pub force: bool,

    /// Disable the interactive fallback and fail if some inputs are wrong or missing.
    #[arg(short, long)]
    pub trust: bool,

    /// Print more detailed information while running.
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug)]
pub struct HighlightedText {
    text: String,
    hex_color: String,
    bold: bool,
    underline: bool,
    italic: bool,
}

fn wait_for_input() -> String {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => {
            return input.trim().to_string();
        }
        Err(_) => {
            // I don't even know what might cause this branch to happen
            println!("Sorry, input reading failed, this shouldn't normally happen.");
            std::process::exit(exitcode::UNAVAILABLE);
        }
    }
}

fn main() {
    let mut conf = CliArgs::parse();
    let cwd = std::env::current_dir().unwrap_or_else(|_| {
        println!("Sorry, can't get current working directory.");
        println!("Something went really wrong, this can only happend when the current directory doesn't exist or you don't have permission to read it.");
        println!("Can't continue further, exiting.");
        std::process::exit(exitcode::UNAVAILABLE);
    });
    let cwd = cwd.as_path();

    if !conf.trust {
        // this loop is interactive way of getting and validating input and output files from user
        while !is_input_ok(&mut conf, cwd) {
            continue;
        }
        while !is_output_ok(&mut conf, cwd) {
            continue;
        }
    }

    let command = "tree-sitter";
    let Some(input_file) = &conf.input.clone() else {
        println!("Can't continue without input file. You probably used \"--trust\" without any input file.");
        std::process::exit(exitcode::USAGE);
    };
    let Some(output_file) = &conf.output else {
        println!("Can't continue without output file. You probably used \"--trust\" without any output file.");
        std::process::exit(exitcode::USAGE);
    };
    let args = ["highlight", "-H", input_file];

    let cmd_output = std::process::Command::new(command).args(args).output();
    match cmd_output {
        Ok(out) => {
            if !out.status.success() {
                println!(
                    "Sorry, the command \"{} {}\" did not succeed and returned status {}.",
                    command,
                    args.join(" "),
                    out.status.code().unwrap_or(-1)
                );
                println!(
                    "Captured stderr: {}",
                    std::str::from_utf8(&out.stderr).unwrap_or("[nothing was captured]")
                );
                std::process::exit(exitcode::NOINPUT);
            } else if !out.stderr.is_empty() {
                let err_msg = String::from_utf8(out.stderr).unwrap_or(String::from("???"));
                if err_msg.contains("No language found for") {
                    println!(
                        "Sorry, TreeSitter couldn't find language parser for the file you entered."
                    );
                    println!("You can inspect all currently usable parsers by executing 'tree-sitter dump-languages'.");
                    println!("Parsers can be downloaded from github.");
                    println!("For more information, please refer to the official TreeSitter highlight documentation at 'https://tree-sitter.github.io/tree-sitter/'.");
                    std::process::exit(exitcode::UNAVAILABLE);
                } else {
                    println!(
                    "WARNING: The command \"{} {}\" finished, but returned error message: \"{}\". ",
                    command,
                    args.join(" "),
                    err_msg
                );
                    println!("The output will probably be not correct.");
                    println!("This is probably not an issue of this program, but rather the external TreeSitter call.");
                }
            }
            if conf.verbose {
                println!(
                    "Successfully called TreeSitter, received {} bytes of data.",
                    out.stdout.len()
                );
            }
            let highlighted_text_pieces = extract_highlighted_pieces(out.stdout, &conf);
            let generated_latex = generate_latex_verbatim(highlighted_text_pieces, &conf);
            if let Err(write_err) = std::fs::write(output_file, generated_latex) {
                println!(
                    "Sorry, there was error while trying to write the output file: {}",
                    write_err
                );
            } else if conf.verbose {
                println!("Successfully written output to \"{}\".", output_file);
            };
        }
        Err(e) => {
            println!(
                "Sorry, the required command \"{} {}\" failed with error message:\n{}.",
                command,
                args.join(" "),
                e
            );
            std::process::exit(exitcode::UNAVAILABLE);
        }
    }
}
