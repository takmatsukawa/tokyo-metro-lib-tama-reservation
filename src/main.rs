use voyager::scraper::Selector;
use voyager::{Collector, Crawler, CrawlerConfig, RequestDelay, Response, Scraper};
use anyhow::Result;
use std::time::Duration;
use futures::StreamExt;
use inquire::{error::InquireError, Select};

struct TamaLibCrawler {
    date_scraper: Selector,
    datetime_scraper: Selector,
}

impl Default for TamaLibCrawler {
    fn default() -> Self {
        Self {
            date_scraper: Selector::parse("table:nth-of-type(2) a").unwrap(),
            datetime_scraper: Selector::parse("table tr:nth-child(5) input[type=radio]").unwrap()
        }
    }
}

#[derive(Debug)]
enum TamaLibState {
    Index,
    Uketsuke(Date),
}

#[derive(Debug)]
struct Date {
    date: String,
    url: String,
    times: Vec<String>
}

impl Scraper for TamaLibCrawler {
    type Output = Date;
    type State = TamaLibState;

    fn scrape(&mut self, response: Response<Self::State>, crawler: &mut Crawler<Self>) -> Result<Option<Self::Output>> {
        let html = response.html();

        if let Some(state) = response.state {
            match state {
                TamaLibState::Index => {
                    for (date, url) in html
                        .select(&self.date_scraper)
                        .map(|el| (el.text().collect::<Vec<_>>().join(""), el.value().attr("href").map(str::to_string).unwrap()))
                        {
                            crawler.visit_with_state(
                                url.clone(),
                                TamaLibState::Uketsuke(Date{
                                    date: date,
                                    url: url,
                                    times: Vec::new()
                                }),
                            );
                        }
                }
                TamaLibState::Uketsuke(mut date) => {
                    for datetime in html
                        .select(&self.datetime_scraper)
                        .filter_map(|el| el.value().attr("value").map(str::to_string))
                        {
                            date.times.push(datetime);
                        }
                    return Ok(Some(date));
                }
            }
        }
        Ok(None)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let config = CrawlerConfig::default().respect_robots_txt().allow_domains_with_delay(
        [
            ("www.library.metro.tokyo.lg.jp", RequestDelay::Fixed(Duration::from_millis(2_000))),
            ("www.shinsei.elg-front.jp", RequestDelay::Fixed(Duration::from_millis(1_000)))
            ],
    );
    let mut collector = Collector::new(TamaLibCrawler::default(), config);

    collector.crawler_mut().visit_with_state(
        "https://www.library.metro.tokyo.lg.jp/guide/tama_library/reservation_tama/",
        TamaLibState::Index
    );

    while let Some(output) = collector.next().await {
        if let Ok(date) = output {
            println!("Date: {:?}", date)
        }
    }

    // let options: Vec<&str> = vec!["Banana", "Apple", "Strawberry", "Grapes", "Lemon", "Tangerine", "Watermelon", "Orange", "Pear", "Avocado", "Pineapple"];

    // let ans: Result<&str, InquireError> = Select::new("Date", options).prompt();

    // match ans {
    //     Ok(choice) => println!("{}! That's mine too!", choice),
    //     Err(_) => println!("There was an error, please try again"),
    // }

    Ok(())
}
