#[derive(Debug)]
struct Order {
    name: String,
    year: u32,
    made_by_phone: bool,
    made_by_mobile: bool,
    made_by_email: bool,
    item_number: u32,
    count: u32,
}

fn create_order_template() -> Order {
    Order {
        name: String::from("Bob"),
        year: 2019,
        made_by_phone: false,
        made_by_mobile: false,
        made_by_email: true,
        item_number: 123,
        count: 0,
    }
}

// Y: Created implementation for Order::new
impl Order {
    pub fn new(
        name: String,
        year: u32,
        made_by_phone: bool,
        made_by_mobile: bool,
        made_by_email: bool,
        item_number: u32,
        count: u32,
    ) -> Self {
        Order {
            name,
            year,
            made_by_phone,
            made_by_mobile,
            made_by_email,
            item_number,
            count,
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn your_order() {
        let order_template = create_order_template();

        // TODO: Create your own order using the update syntax and template above!
        // let your_order = "Hacker in Rust", 2019, false, false, true, 123, 1

        /* Y: expected answer : added _ to remove warning. */
        let _your_order = Order {
            name: String::from("Hacker in Rust"),
            count: 1,
            ..create_order_template()
        };

        /* Y: Addvanced way to solve. */
        let your_order = Order::new(
            String::from("Hacker in Rust"),
            2019,
            false,
            false,
            true,
            123,
            1,
        );

        assert_eq!(your_order.name, "Hacker in Rust");
        assert_eq!(your_order.year, order_template.year);
        assert_eq!(your_order.made_by_phone, order_template.made_by_phone);
        assert_eq!(your_order.made_by_mobile, order_template.made_by_mobile);
        assert_eq!(your_order.made_by_email, order_template.made_by_email);
        assert_eq!(your_order.item_number, order_template.item_number);
        assert_eq!(your_order.count, 1);
    }
}
