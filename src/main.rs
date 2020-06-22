#[allow(unused_imports)]

use std::path::PathBuf;
use structopt::StructOpt;
use anyhow::Result;

use std::string::String;

use async_std::prelude::*;
use async_std::{task::block_on, io,io::BufReader};

#[derive(Debug, StructOpt)]
#[structopt(name = "lsrt", about = "lexical sorter")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,
    #[structopt(short = "k", long = "key")] 
    key: i8,
    #[structopt(short = "H", long ="human")]
    human: bool,
    #[structopt(short = "F", long = "fieldsep")]
    sep: String,

    file: Vec<PathBuf>,

}

async fn fillgrid(sep: &String) -> Result<Vec<Vec<String>>> {
    let mut grid: Vec<Vec<String>> = vec![];
    let stdin = io::stdin();
    let reader = BufReader::new(stdin);
    let mut lines = reader.lines();
    while let Some(line) = lines.next().await {
        let row:Vec<String> = line
          .unwrap()
          .split(sep)
          .skip_while(|s| s == &"")
          .map(|s| s.trim().to_string())
          .collect();
        println!("{:?}",row);
        grid.push(row);
    }
    Ok(grid)

}
fn main() -> Result<()> {
    let opt = Opt::from_args();
    if opt.file.len() == 0 {
        let fut = fillgrid(&opt.sep);
        let buf = block_on(fut);
        println!("{:?}",buf.unwrap());
    }

    Ok(())
}