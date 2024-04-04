use clap::{Args, Parser, Subcommand};
use std::path::Path;

// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // 默认name就是工程名，如果指定 name= xxx
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Subcommand)]
pub enum SubCommand {
    Csv(CsvOpts),
}

/// Show CSV, or convert CSV to other formats
#[derive(Args, Debug)]
pub struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    /// Output file path
    #[arg(short, long, default_value = "output.json")] // "output.json".into()
    pub output: String,

    /// Delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    /// CSV has header or not
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

/// 转换函数Fn(&str) -> Result<T, E>
/// 这里只是用于检验文件是否存在
fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() {
        Ok(filename.to_string())
    } else {
        Err("File does not exist")
    }
}
