// Event struct, details

pub struct Event {
    pub name: String,
    pub date: String,
    pub total_seats: i32,
    pub price_per_seat: f64,
}

impl Event {
    /* Y:
     *    Constructor function
     *--------------------------------------
     *  |  pub fn new(...) -> Self{Self}   |
     *--------------------------------------
     *
     */

    pub fn new(name: String, date: String, total_seats: i32, price_per_seat: f64) -> Self {
        Self {
            name,
            date,
            total_seats,
            price_per_seat,
        }
    }
    pub fn avialable_seats(&self) -> i32 {
        self.total_seats
    }
}
