use std::collections::HashMap;
use crate::config::Config;
use reqwest::Url;

pub struct Stock {
    pub symbol: String,
    pub current_price: f64,
    pub opening_price: f64
}

impl ToString for Stock {
    fn to_string(&self) -> String {
        return format!("{} current_price: {} opening_price: {}", self.symbol, self.current_price, self.opening_price);
    }
}

pub fn refresh_symbols(config: &Config) -> Vec<Stock> {
    let mut stocks = Vec::<Stock>::new();
    for symbol in config.symbols.iter() {
        let stock = get_stock_info(&config.token, &symbol);
        stocks.push(stock);
    }
    return stocks;
}

fn get_stock_info(token: &str, symbol: &str) -> Stock {
    let formatted_url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", symbol, token);
    let url = Url::parse(&formatted_url).expect("Could Not Parse URL");
    let resp = reqwest::blocking::get(url).unwrap().json::<HashMap<String, f64>>().expect("Could Not Get Stocks");

    let current_price = resp["c"];
    let opening_price = resp["o"];
    return Stock{
        symbol: symbol.to_string(),
        current_price: current_price,
        opening_price: opening_price
    }
}
