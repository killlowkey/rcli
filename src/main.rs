use clap::Parser;
use rcli::{process_csv, Opts, Subcommand};

// rcli csv -i input.csv -o output.json -h -d ','
// cargo run -- csv -i assets/juventus.csv
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = opts
                .output
                .unwrap_or_else(|| format!("output.{}", opts.format)); // 使用 copy 传值
            process_csv(&opts.input, output, opts.format)?
        }
    }

    Ok(())
}
