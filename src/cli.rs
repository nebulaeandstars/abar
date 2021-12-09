use std::env;
use std::io::Write;
use std::net::TcpStream;

use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

pub fn process_args() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        return;
    }

    let program = args[0].clone();
    let mut opts = Options::new();

    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        },
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        std::process::exit(0);
    }

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:2227") {
        let data = args[1..].join(" ");
        stream.write(&data.as_bytes()).unwrap();
        std::process::exit(0);
    }
}
