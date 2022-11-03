use core::time;
use std::io::{self, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::mpsc::{self, Sender};
use std::thread;
use structopt::StructOpt;

const MAX: u16 = 65535;

#[derive(Debug, StructOpt)]
#[structopt(name = "port-sniffer", about = "A host port sniffer.")]
struct Arguments {
    #[structopt(short = "a", long, default_value = "127.0.0.1")]
    ipaddr: String,

    #[structopt(short, long, default_value = "2000")]
    threads: u16,

    #[structopt(short = "o", long, default_value = "5")]
    timeout: u64,
}

fn scan(
    tx: Sender<u16>,
    start_port: u16,
    host: String,
    num_threads: u16,
    timeout: u64,
) -> anyhow::Result<()> {
    let mut port: u16 = start_port + 1;
    let timeout = time::Duration::from_secs(timeout);
    loop {
        let addr = format!("{}:{}", host, port);
        let mut addr = addr.to_socket_addrs()?;
        let addr = addr.next().ok_or(anyhow::anyhow!("unknown error"))?;
        if TcpStream::connect_timeout(&addr, timeout).is_ok() {
            print!(".");
            io::stdout().flush()?;
            tx.send(port)?;
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let arguments = Arguments::from_args();

    let num_threads = arguments.threads;
    let (tx, rx) = mpsc::channel();
    for i in 0..num_threads {
        let tx = tx.clone();
        let host = arguments.ipaddr.clone();

        thread::spawn(move || {
            let _ = scan(tx, i, host, num_threads, arguments.timeout);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!();
    out.sort_unstable();
    for v in out {
        println!("{} is open", v);
    }

    Ok(())
}
