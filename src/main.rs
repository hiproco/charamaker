use rand;
use std::{
    io::{self, Write},
    path::Path,
};

fn main() -> Result<(), io::Error> {
    eprintln!("{:?}", read_character("test.txt"));
    let mut out = std::fs::File::create("test.txt")?;
    write!(&mut out, "{}", gen_stats("stats.txt"))
}

fn gen_stats<'a, P: AsRef<Path>>(path: P) -> String {
    let stats = std::fs::read_to_string(path).unwrap();
    let stats = stats.lines();
    stats
        .map(|stat| {
            format!(
                "{}:{}\n",
                stat,
                (0..5).map(|_| rand::random::<u8>() / 5).sum::<u8>()
            )
        })
        .collect()
}

fn read_character(path: impl AsRef<std::path::Path>) -> Result<Vec<(String, u8)>, io::Error> {
    std::fs::read_to_string(path)?
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
