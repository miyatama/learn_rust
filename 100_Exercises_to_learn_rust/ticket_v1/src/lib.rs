pub fn main_logic() {
    println!("hello");
}

struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn is_open(self) -> bool {
        self.status == "Open"

    }

    pub fn sample() -> Self {
        Ticket {
            title: "Build a ticket system".into(),
            description: "kanban board".into(),
            status: "Open".into(),
        }
    }
}


struct Configration {
    version: u32,
    active: bool,
}

/**
 * ex) let default_config = Configuration::default();
 */
impl Configration {
    fn default() -> Configration {
        Configration {
            version: 0,
            active: false,
        }
    }
}

struct Order  {
    price: u32,
    quantity: u32,
}

impl Order {
    fn is_available(self) -> bool {
        self.quantity > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };
        assert!(order.is_available());
    }

    #[test]
    fn test_order_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };
        assert!(!order.is_available());
    }
}