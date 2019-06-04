extern crate structopt;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Histogram of words in a file",
    about = ""
)]
struct Cli {
    /// file
    #[structopt(name = "FILE", parse(from_str))]
    file: String,
}

fn main() {
    let opt = Cli::from_args();

    println!("Computing histogam of {}", opt.file);
    let fp = File::open(opt.file).unwrap();

    let file = BufReader::new(&f);
    for (num, line) in file.lines().enumerate() {
        let words = line.unwrap().split(/\s+/);
        println("{}", words);
    }
}
