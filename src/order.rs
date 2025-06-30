use core::num::dec2flt::decimal::Decimal;

enum OrderType {
    Buy,
    Sell,
}

struct Order {
    quantity: u32,
    price: Decimal,
    order_type: OrderType,
}

impl Order {
    fn new(quantity: u32, price: Decimal, order_type: OrderType) -> Order {
        Order {
            quantity,
            price,
            order_type,
        }
    }
}