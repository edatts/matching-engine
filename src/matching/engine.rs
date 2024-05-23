use super::orderbook::OrderBook;
use std::collections::HashMap;

pub struct MatchingEngine {
    orderbooks: HashMap<Market, OrderBook>
}

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct Market {
    base: String,
    quote: String,
    decimals: u32,
}

// TODO: Replace strings with custom asset types...
impl Market {
    pub fn new(base: String, quote: String, decimals: u32) -> Market {
        return Market { base, quote, decimals }
    }

    pub fn to_string(&self) -> String {
        return format!("{}_{}", self.base, self.quote)
    }
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_market(&mut self, market: Market) {
        self.orderbooks.insert(market.clone(), OrderBook::new());

        println!("Added new orderbook for market {:?}", market.to_string())
    }
}