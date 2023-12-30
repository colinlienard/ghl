use std::process;

use snp::Config;

fn main() {
    let config = Config::ask().unwrap_or_else(|_| {
        eprintln!("An error occured");
        process::exit(1);
    });

    println!("{} {}", config.pr_name, config.branch);
}
