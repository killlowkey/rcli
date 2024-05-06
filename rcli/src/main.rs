use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use clap::Parser;
use rcli::{
    get_content, get_reader, process_csv, process_decode, process_encode, process_gen_pass,
    process_text_key_generate, process_text_sign, process_text_verify, Base64Subcommand, Opts,
    Subcommand, TextSubcommand,
};
use std::fs;
use zxcvbn::zxcvbn;

// rcli csv -i input.csv -o output.json -h -d ','
// cargo run -- csv -i assets/juventus.csv
// cargo run -- base64 encode -i filename
// cargo run -- base64 decode -i filename
// cargo run -- text sign -k fixtures/blake3.txt
// cargo run -- text verify -k fixtures/blake3.txt --sig 7gw0oiCTB2E5YOJx5sz2poH9ZJsdvHhmTgnaC2aX4Wo
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
            let ret = process_gen_pass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.number,
                opts.symbol,
            )?;
            println!("{}", ret);

            // output password strength in stderr
            let estimate = zxcvbn(&ret, &[])?;
            eprintln!("Password strength: {}", estimate.score());
        }
        Subcommand::Base64(subcommand) => match subcommand {
            Base64Subcommand::Encode(opt) => {
                process_encode(&opt.input, opt.format)?;
            }
            Base64Subcommand::Decode(opt) => {
                process_decode(&opt.input, opt.format)?;
            }
        },
        Subcommand::Text(subcommand) => match subcommand {
            TextSubcommand::Sign(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let sig = process_text_sign(&mut reader, &key, opts.format)?;
                // base64 output
                let encoded = URL_SAFE_NO_PAD.encode(sig);
                println!("{}", encoded);
            }
            TextSubcommand::Verify(opts) => {
                let mut reader = get_reader(&opts.input)?;
                let key = get_content(&opts.key)?;
                let decoded = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                let verified = process_text_verify(&mut reader, &key, &decoded, opts.format)?;
                if verified {
                    println!("✓ Signature verified");
                } else {
                    println!("⚠ Signature not verified");
                }
            }
            TextSubcommand::Generate(opts) => {
                let key = process_text_key_generate(opts.format)?;
                for (k, v) in key {
                    fs::write(opts.output_path.join(k), v)?;
                }
            }
        },
    }

    Ok(())
}
