use rand;
use std::{
    fs,
    io::{self, Write},
    path::Path,
};

// -t <template file>
// -o <output file>
fn main() -> Result<(), io::Error> {
    let arg = clinterface::args(&["-t", "-o", "--template", "--output"]);
    let options_arg = |option: &[&str], default: String| {
        arg.iter()
            .find_map(|a| option.contains(&a.option.as_str()).then(|| a.args.clone()))
            .unwrap_or_else(|| vec![default])
    };
    let path = options_arg(&["-t", "--template"], "test.txt".to_string());
    let template = options_arg(&["-o", "--output"], "stats.txt".to_string());
    eprintln!("{path:?},{template:?}");
    eprintln!("{:?}", read_character(AsRef::<str>::as_ref(&path[0])));
    let mut out = fs::File::create(AsRef::<str>::as_ref(&path[0]))?;
    write!(&mut out, "{}", gen_stats("stats.txt"))
}

mod clinterface {

    type Arg = String;
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct OptionArg {
        pub option: Arg,
        pub args: Vec<Arg>,
    }

    pub fn args<S: AsRef<str>>(valid_options: &[S]) -> Vec<OptionArg> {
        let is_option = |a: &String| a.starts_with("--") || a.starts_with('-');
        let options = std::env::args()
            .skip(1)
            .filter(is_option)
            .collect::<Vec<_>>();
        let args = std::env::args().skip(1).collect::<Vec<_>>();
        let split = args.split(is_option).collect::<Vec<_>>();
        split
            .into_iter()
            .zip(std::iter::once(String::new()).chain(options))
            .map(|(args, option)| OptionArg {
                option,
                args: args.to_vec(),
            })
            .collect()
    }
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
