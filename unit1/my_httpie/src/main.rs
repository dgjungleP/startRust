use anyhow::{anyhow, Result};
use clap::Parser;
use reqwest::Url;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Jungle")]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Parser, Debug)]
enum Subcommand {
    Get(Get),
    Post(Post),
}
#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str= parse_url))]
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str= parse_url))]
    url: String,
    #[clap(parse(try_from_str= parse_kv_pair))]
    body: Vec<KvPair>,
}

fn parse_url(url: &str) -> Result<String> {
    let _url: Url = url.parse()?;
    Ok(url.into())
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}
impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Fail to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).into(),
            v: (split.next().ok_or_else(err)?).into(),
        })
    }
}

fn parse_kv_pair(kv: &str) -> Result<KvPair> {
    Ok(kv.parse()?)
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
