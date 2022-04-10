use voyager::scraper::Selector;
use voyager::{Collector, Crawler, CrawlerConfig, RequestDelay, Response, Scraper};
use anyhow::Result;
use std::time::Duration;
use futures::StreamExt;
use inquire::{error::InquireError, Select};
use reqwest;
use std::io::{stdout, Write};

use curl::easy::Easy;
use encoding_rs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut easy = Easy::new();
    easy.url("https://www.shinsei.elg-front.jp/tokyo2/uketsuke/form.do?id=1648881164572").unwrap();
    easy.write_function(|data| {
        let (res, _, _) = encoding_rs::SHIFT_JIS.decode(data);

        stdout().write_all(res.as_bytes()).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());


    Ok(())
}
