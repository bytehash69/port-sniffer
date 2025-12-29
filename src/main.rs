#![allow(dead_code)]
use std::{
    env,
    io::{self, Write},
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
    sync::mpsc::{Sender, channel},
    thread,
};

const MAX: u16 = 50_000;

struct Arguments {
    flag: String,
    ip: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            println!(
                "Usage: -t to select how many threads you want \n\r -h to -help to show this help message"
            );
            return Err("help");
        } else if args.len() > 4 {
            return Err("too many arguments to snidd :(");
        };

        let f = args[1].clone();
        if let Ok(ip) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ip,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!(
                    "Usage: cargo run -- address \n -t to select how many threads you want \n\r -h to -help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments :(");
            } else if flag.contains("-t") {
                let ip = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("not a valid IPADDR; must be IPv4 or IPv6"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number"),
                };

                return Ok(Arguments { flag, ip, threads });
            } else {
                return Err("invalid syntax");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }
        port += num_threads
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0)
        } else {
            println!("{}", err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ip;
    let (tx, rx) = channel();

    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || scan(tx, i, addr, num_threads));
    }

    let mut out = vec![];
    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");
    out.sort();
    for v in out {
        println!("{} is open", v);
    }
}
