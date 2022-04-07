use voyager::scraper::Selector;
use voyager::{Collector, Crawler, CrawlerConfig, RequestDelay, Response, Scraper};
use anyhow::Result;
use std::time::Duration;
use futures::StreamExt;
use reqwest::Url;

struct TamaLibCrawler {
    date_scraper: Selector
}

impl Default for TamaLibCrawler {
    fn default() -> Self {
        Self {
            date_scraper: Selector::parse("table:nth-of-type(2) a").unwrap()
        }
    }
}

#[derive(Debug)]
enum TamaLibState {
    Index,
    Uketsuke,
}

#[derive(Debug)]
struct Date {
    date: String,
}

impl Scraper for TamaLibCrawler {
    type Output = Date;
    type State = TamaLibState;

    fn scrape(&mut self, response: Response<Self::State>, crawler: &mut Crawler<Self>) -> Result<Option<Self::Output>> {
        let html = response.html();

        if let Some(state) = response.state {
            match state {
                TamaLibState::Index => {
                    for date in html
                        .select(&self.date_scraper)
                        .map(|el| el.text().collect::<Vec<_>>().join(""))
                        {
                            println!("{}", date);
                        }
                }
                TamaLibState::Uketsuke => {
                    println!("Uketsuke");
                }
            }
        }
        Ok(None)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = CrawlerConfig::default().allow_domain_with_delay(
        "www.library.metro.tokyo.lg.jp",
         RequestDelay::Fixed(Duration::from_millis(2_000))
    );
    let mut collector = Collector::new(TamaLibCrawler::default(), config);

    collector.crawler_mut().visit_with_state(
        "https://www.library.metro.tokyo.lg.jp/guide/tama_library/reservation_tama/",
        TamaLibState::Index
    );


    while let Some(output) = collector.next().await {
        if let Ok(post) = output {
            println!("Post: {:?}", post)
        }
    }

    Ok(())
}
