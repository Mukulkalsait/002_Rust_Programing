pub mod booking;
pub mod event;
pub mod ticket;
pub mod user;
pub mod utils;

// Y:
// program_one/
//  ├── mod.rs          // Re-exports all modules
//  ├── event.rs        // Event struct, details
//  ├── ticket.rs       // Ticket struct, seat, price
//  ├── user.rs         // User struct, name, contact
//  ├── booking.rs      // Booking logic (buy, cancel, list)
//  ├── utils.rs        // Helpers (date/time formatting, ID generation)
