use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_gen_pass, Base64Subcommand, Opts,
    Subcommand,
};

// rcli csv -i input.csv -o output.json -h -d ','
// cargo run -- csv -i assets/juventus.csv
// cargo run -- base64 encode -i filename
// cargo run -- base64 decode -i filename
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(opts) => {
            let output = opts
                .output
                .unwrap_or_else(|| format!("output.{}", opts.format)); // 使用 copy 传值
            process_csv(&opts.input, output, opts.format)?
        }
        Subcommand::GenPass(opts) => {
            process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
        }
        Subcommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(opt) => {
                process_encode(&opt.input, opt.format)?;
            }
            Base64Subcommand::Decode(opt) => {
                process_decode(&opt.input, opt.format)?;
            }
        },
    }

    Ok(())
}
