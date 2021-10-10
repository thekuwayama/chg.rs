use anyhow::Result;
use std::io::BufReader;

mod concurrent_http_get;

fn main() -> Result<()> {
    let mut reader = BufReader::new(std::io::stdin());
    concurrent_http_get::concurrent_http_get(&mut reader)
}
