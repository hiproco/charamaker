use rand;
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

// -t <template file>
// -o <output file>
fn main() -> Result<(), io::Error> {
    let options = options();
    if !options.is_empty() {
        let retrive = options.iter().map(|(i, _)| i + 1).collect::<Vec<_>>();
        let option_args = std::env::args()
            .enumerate()
            .filter(|(i, _)| retrive.contains(i))
            .map(|(_, a)| a)
            .collect::<Vec<_>>();
        eprintln!("{retrive:?}");
        eprintln!("{option_args:?}");
    }
    let path = "test.txt"; //args.nth(1).unwrap_or_else(|| "test.txt".to_string());
    let template = "stats.txt"; //args.nth(2).unwrap_or_else(|| "stats.txt".to_string());
    eprintln!("{:?}", read_character(AsRef::<str>::as_ref(path)));
    let mut out = fs::File::create(AsRef::<str>::as_ref(path))?;
    write!(&mut out, "{}", gen_stats("stats.txt"))
}

fn options() -> Vec<(usize, String)> {
    std::env::args()
        .enumerate()
        .filter(|(i, a)| a.starts_with("--") || a.starts_with('-'))
        .collect()
}

fn gen_stats<'a, P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|stat| {
            format!(
                "{}:{}\n",
                stat,
                (0..5).map(|_| rand::random::<u8>() / 5).sum::<u8>()
            )
        })
        .collect()
}

fn read_character<P: AsRef<Path>>(path: P) -> Result<Vec<(String, u8)>, io::Error> {
    fs::read_to_string(path)?
        .lines()
        .try_fold(Vec::new(), |mut v, line| {
            use std::io::{Error, ErrorKind};
            let (stat, val) = line
                .split_once(":")
                .ok_or(Error::new(ErrorKind::Other, "format error"))?;
            let val = val
                .parse::<u8>()
                .map_err(|_| Error::new(ErrorKind::Other, "value error"))?;
            v.push((stat.to_string(), val));
            Ok(v)
        })
}
