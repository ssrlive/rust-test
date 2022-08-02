use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

const MAX: u16 = 65535;

struct Arguments {
    _flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            Ok(Arguments {
                _flag: String::from(""),
                ipaddr,
                threads: 4,
            })
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") && args.len() == 2 {
                println!(
                    "Usage: -j <thread_count> <target_IP_address>
                \n\r       -h or --help to show this help message"
                );
                Err("help")
            } else if flag.contains("-h") || flag.contains("-help") {
                Err("too many arguments")
            } else if flag.contains("-j") {
                let addr_str = match args.get(3) {
                    None => return Err("please assign an IP address"),
                    Some(a) => a,
                };
                let ipaddr = match IpAddr::from_str(addr_str) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };
                Ok(Arguments {
                    threads,
                    _flag: flag,
                    ipaddr,
                })
            } else {
                Err("invalid syntax")
            }
        }
    }
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if !err.contains("help") {
            eprintln!("{} problem parsing arguments: {}", program, err);
        }
        process::exit(0);
    });

    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
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
}
