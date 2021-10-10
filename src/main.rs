#[macro_use]
extern crate clap;

mod concurrent_http_get;

use anyhow::Result;
use clap::{App, Arg};
use std::io::BufReader;

fn main() -> Result<()> {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("mw")
                .long("multi-worker")
                .required(false)
                .takes_value(false),
        );

    let mut reader = BufReader::new(std::io::stdin());
    if app.get_matches().is_present("mw") {
        concurrent_http_get::mw_concurrent_http_get(&mut reader)
    } else {
        concurrent_http_get::pipe_concurrent_http_get(&mut reader)
    }
}
