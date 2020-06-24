#![feature(main)]
use anyhow::Result;
#[allow(unused_imports)]
use std::path::PathBuf;
use structopt::StructOpt;

use std::marker::Unpin;
use std::string::String;

use async_std::fs::File;
use async_std::prelude::*;
use async_std::{io, io::BufReader, io::Read, task::block_on};
#[derive(Debug, StructOpt)]
#[structopt(name = "lsrt", about = "lexical sorter")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short = "k", long = "key")]
    key: i8,
    #[structopt(short = "H", long = "human")]
    human: bool,
    #[structopt(short = "F", long = "fieldsep")]
    sep: char,
    #[structopt(short = "l", long = "lines")]
    lines: u32,

    file: Vec<PathBuf>,
}

async fn fillgrid<R: Read + Unpin>(
    fd: &mut io::BufReader<R>,
    sep: &char,
) -> Result<Vec<Vec<String>>> {
    let mut grid: Vec<Vec<String>> = vec![];
    let mut lines = fd.lines();
    while let Some(line) = lines.next().await {
        let row: Vec<String> = line
            .unwrap()
            .split(|c| c == *sep || c == '\t')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim().to_string())
            .collect();
        grid.push(row);
    }
    Ok(grid)
}

// #[main]
async fn _main_() -> Result<()> {
    let opt = Opt::from_args();
    let buf = match opt.file.len() {
        0 => {
            let fd = io::stdin();
            let mut reader = BufReader::new(fd);
            let parsed_fut = fillgrid(&mut reader, &opt.sep);
            let buf = block_on(parsed_fut);
            buf
        }
        1 => {
            let file = File::open(&opt.file[0]).await;
            let mut reader = BufReader::new(file);
            let parsed_fut = fillgrid(&mut reader, &opt.sep);
            let buf = block_on(parsed_fut);
            buf
        }

        _ => Err(anyhow::anyhow!("no parsable strings found")),
    };
    println!("{:?}", buf);

    Ok(())
}
fn main() -> Result<()> {
    block_on(_main_());
    Ok(())
}
