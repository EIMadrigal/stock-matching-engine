use std::collections::HashMap;

struct OrderBook {
    // price -> orders at that price
    buy_orders: HashMap<Decimal, Vec<Order>>,
    sell_orders: HashMap<Decimal, Vec<Order>>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            buy_orders: HashMap::new(),
            sell_orders: HashMap::new(),
        }
    }

    fn add_order(&self, order: Order) {
        let order_price = order.price;
        match order.order_type {
            OrderType::Buy => {
                match self.buy_orders.get(order_price) {
                    Some(orders) => {
                        orders.push(order);
                    }
                    None => {
                        self.buy_orders.insert(
                            order_price,
                            vec![order],
                        );
                    }
                }
            }
            OrderType::Sell => {
                match self.sell_orders.get(order_price) {
                    Some(orders) => {
                        orders.push(order);
                    }
                    None => {
                        self.sell_orders.insert(
                            order_price,
                            vec![order],
                        );
                    }
                }
            }
        }
    }

    fn get_best_buy_price() {}

    fn get_best_sell_price() {}


}