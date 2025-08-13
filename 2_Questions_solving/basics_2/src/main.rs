use basics_2::program_one::event::Event;

fn event_ticket_booking_system() {
    let event = Event::new(
        "Music Consert".to_string(),
        "2025-08-06".to_string(),
        150,
        0,
        499.99,
    );
    println!("Available Seats are : {}", event.available_seats());
    // println!("Available Seats are : {}", event.book_seats());
}
fn main() {
    event_ticket_booking_system();
}
