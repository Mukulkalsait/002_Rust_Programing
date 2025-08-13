// Event struct, details

pub struct Event {
    pub name: String,
    pub date: String,
    pub total_seats: u32,
    pub booked_seats: u32,
    pub price_per_seat: f64,
}

impl Event {
    /* B: Constructor function
     *   ------------------------------------
     *   |  pub fn new(...) -> Self{Self}   |
     *   ------------------------------------
     */

    /* Y: Constructor new*/
    pub fn new(
        name: String,
        date: String,
        total_seats: u32,
        booked_seats: u32,
        price_per_seat: f64,
    ) -> Self {
        Self {
            name,
            date,
            total_seats,
            booked_seats,
            price_per_seat,
        }
    }

    /* Y: Find Available Seats*/
    pub fn available_seats(&self) -> u32 {
        self.total_seats - self.booked_seats
    }

    /* Y: Book Seats */
    pub fn book_seats(&mut self, need_seats: u32) -> (String, u32) {
        if self.available_seats() >= need_seats {
            self.booked_seats += need_seats;
            (String::from("Booking Success !"), need_seats)
        } else {
            (String::from("Not Enough Seats"), self.available_seats())
        }
    }

    /* Y: Cancel Seats */
    pub fn cancle_seats(&mut self, seats_to_cancle: u32) -> (String, u32) {
        if self.booked_seats >= seats_to_cancle {
            self.booked_seats -= seats_to_cancle;
            (
                String::from("Seats Successfully Canceled "),
                seats_to_cancle,
            )
        } else {
            (
                String::from("Booked Seats Are Smaller Than the seats you are trying to cancle"),
                self.booked_seats,
            )
        }
    }
}
