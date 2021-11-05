use std::io::Write;
use std::{fs::File, thread};
use structopt::StructOpt;

/// Exercises the file_open LSM hook very quickly in parallel
#[derive(StructOpt)]
struct Opt {
    /// Number of threads to spin up
    #[structopt(short, long, default_value = "64")]
    threads: u64,
    /// Open calls per thread
    #[structopt(short, long, default_value = "10000")]
    open_calls: u64,
}

fn main() {
    let opt = Opt::from_args();

    let mut f = File::create("/tmp/test").expect("Failed to create file");
    f.write("foo!".as_bytes()).expect("Failed to write");

    let join_handles: Vec<_> = (0..opt.threads)
        .map(|_| {
            thread::spawn(move || {
                for _ in 0..opt.open_calls {
                    let _ = File::open("/tmp/test").unwrap();
                }
            })
        })
        .collect();

    for handle in join_handles {
        handle.join().unwrap()
    }
}
