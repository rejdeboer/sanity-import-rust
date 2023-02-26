extern crate sanity;
mod import;
use std::env;
use clap::Parser;

#[derive(Parser, Default, Debug)]
#[clap(author = "Rick de Boer", version, about)]
/// An implementation of the Sanity import CLI in Rust
struct Cli {
    project_id: String,
    dataset: String,
    file_path: std::path::PathBuf,
    #[clap(short, long)]
    token: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let token = &args.token.unwrap_or_else(|| {
        match env::var("SANITY_IMPORT_TOKEN") {
            Ok(v) => { v },
            Err(_e) => panic!(
                "No import token set, you can set it via the -t flag or the SANITY_IMPORT_TOKEN environment variable"
            )
        }
    });

    let sn = sanity::create(
        &args.project_id, 
        &args.dataset, 
        token, 
        false
    );

    import::import_data(&args.file_path, sn);
}

