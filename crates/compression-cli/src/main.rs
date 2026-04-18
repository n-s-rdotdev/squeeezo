use std::process::ExitCode;

use clap::{Parser, ValueEnum};
use compression_core::{
    compress_pdf, CompressionRequest, CompressionSource, GhostscriptAdapter,
};

#[derive(Clone, Debug, ValueEnum)]
enum CliSource {
    Desktop,
    FinderAction,
    Cli,
}

impl From<CliSource> for CompressionSource {
    fn from(value: CliSource) -> Self {
        match value {
            CliSource::Desktop => CompressionSource::Desktop,
            CliSource::FinderAction => CompressionSource::FinderAction,
            CliSource::Cli => CompressionSource::Cli,
        }
    }
}

#[derive(Debug, Parser)]
#[command(name = "squeeezo-compress")]
struct Cli {
    #[arg(long)]
    input: String,
    #[arg(long, default_value = ".compressed")]
    suffix: String,
    #[arg(long)]
    json: bool,
    #[arg(long, value_enum, default_value = "cli")]
    source: CliSource,
}

fn main() -> ExitCode {
    let args = Cli::parse();
    let result = compress_pdf(
        &CompressionRequest {
            input_path: args.input,
            source: args.source.into(),
            suffix: Some(args.suffix),
        },
        &GhostscriptAdapter,
    );

    if args.json {
        println!(
            "{}",
            serde_json::to_string_pretty(&result).expect("json serialization")
        );
    } else {
        println!("{result:#?}");
    }

    if matches!(result.status, compression_core::CompressionStatus::Failed) {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
