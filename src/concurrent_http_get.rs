use anyhow::Result;
use std::io::BufRead;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{spawn, JoinHandle};

pub fn concurrent_http_get<R: BufRead>(reader: &mut R) -> Result<()> {
    let (sender, receiver) = channel();
    let (r1, h1) = send_http_get(receiver);
    let (r2, h2) = map_response_body(r1);

    for url in reader.lines() {
        if url.is_err() || sender.send(url.unwrap()).is_err() {
            break;
        }
    }

    drop(sender);
    h1.join().unwrap().unwrap();
    h2.join().unwrap().unwrap();
    for s in r2 {
        println!("{}", s);
    }

    Ok(())
}

fn send_http_get(
    urls: Receiver<String>,
) -> (
    Receiver<reqwest::blocking::Response>,
    JoinHandle<Result<()>>,
) {
    let (sender, receiver) = channel();

    let handle = spawn(move || {
        for url in urls {
            let resp = reqwest::blocking::get(url)?;
            if sender.send(resp).is_err() {
                break;
            }
        }

        Ok(())
    });

    (receiver, handle)
}

fn map_response_body(
    resps: Receiver<reqwest::blocking::Response>,
) -> (Receiver<String>, JoinHandle<Result<()>>) {
    let (sender, receiver) = channel();

    let handle = spawn(move || {
        for resp in resps {
            let resp = resp.text()?;
            if sender.send(resp).is_err() {
                break;
            }
        }

        Ok(())
    });

    (receiver, handle)
}
