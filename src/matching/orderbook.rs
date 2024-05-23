
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum OrderType {
    GTC,
    GTT,
    IOC, 
    FOK,
}

#[derive(Debug, Clone, Copy)]
pub enum OrderStatus {
    Live,
    Filled,
    PartiallyFilled,
    Cancelled,
    Stopped,
}

#[derive(Debug, Clone, Copy)]
pub struct Order {
    price: u128,
    size: u128,
    is_bid: bool,
    order_type: OrderType,
    status: OrderStatus,
}

impl Order {
    pub fn new(price: u128, size: u128, is_bid: bool, order_type: OrderType, status: OrderStatus) -> Order {
        return Order{price, size, is_bid, order_type, status}
    }
}

// We have the choice of storing price levels
// in a hashmap or in an ordered array/vector
#[derive(Debug)]
pub struct OrderBook {
    asks: HashMap<u128, PriceLevel>,
    bids: HashMap<u128, PriceLevel>
}

impl OrderBook {
    pub fn new() -> OrderBook {
        return OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new()
        }
    }

    pub fn add_order(&mut self, order: Order) {
        match order.is_bid {
            true => {
                let price_level = self.bids.get_mut(&order.price);
                match price_level {
                    Some(price_level) => price_level.add_order(order),
                    None => {
                        let mut price_level = PriceLevel::new(order.price);
                        price_level.add_order(order);
                        self.bids.insert(order.price, price_level);
                    }
                }
            },
            false => {
                let price_level = self.asks.get_mut(&order.price);
                match price_level {
                    Some(price_level) => price_level.add_order(order),
                    None => {
                        let mut price_level = PriceLevel::new(order.price);
                        price_level.add_order(order);
                        self.bids.insert(order.price, price_level);
                    }
                }
            } 
        }
    }
}


#[derive(Debug)]
struct PriceLevel {
    price: u128,
    orders: Vec<Order>,
}

impl PriceLevel {
    fn new(price: u128) -> PriceLevel {
        return PriceLevel{price, orders: Vec::new()}     
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }
}