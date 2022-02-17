use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};

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

#[derive(Debug, PartialEq)]
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
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    let client = Client::new();
    let resut = match opts.subcmd {
        Subcommand::Get(ref args) => get(client, args).await?,
        Subcommand::Post(ref args) => post(client, args).await?,
    };
    Ok(resut)
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

fn print_status(resp: &Response) {
    let status = format!("{:?}{}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_hreaders(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}:{:?}", name.to_string().green(), value);
    }
    println!("\n");
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan());
        }
        _ => println!("{}", body),
    }
}
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_hreaders(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("http://httpbin.org/post").is_ok());
    }
    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into()
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into()
            }
        );
    }
}
