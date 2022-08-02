use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::{
    error,
    io::{self, Write},
};
use structopt::StructOpt;

const MAX: u16 = 65535;

#[derive(Debug, StructOpt)]
#[structopt(name = "port-sniffer", about = "A host port sniffer.")]
struct Arguments {
    #[structopt(short = "a", long, default_value = "127.0.0.1")]
    ipaddr: String,

    #[structopt(short, long, default_value = "2000")]
    threads: u16,
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        if TcpStream::connect((addr, port)).is_ok() {
            print!(".");
            io::stdout().flush().unwrap();
            tx.send(port).unwrap();
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let arguments = Arguments::from_args();
    let addr = IpAddr::from_str(&arguments.ipaddr)?;

    let num_threads = arguments.threads;
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
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
