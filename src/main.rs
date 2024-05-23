// We can start building out some of the core types in main and then
// move them into their own files later.
mod matching;
use matching::orderbook::{Order, OrderType, OrderStatus, OrderBook};
use matching::engine::{MatchingEngine, Market};

fn main() {

    let order: Order = Order::new(6700, 5, true, OrderType::GTC, OrderStatus::Live);
    let order_2: Order = Order::new(6700, 3, true, OrderType::GTC, OrderStatus::Live);

    let mut orderbook: OrderBook = OrderBook::new();
    orderbook.add_order(order);
    orderbook.add_order(order_2);

    println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let market = Market::new("BTC".to_string(), "USD".to_string(), 2);
    engine.add_market(market);

}
