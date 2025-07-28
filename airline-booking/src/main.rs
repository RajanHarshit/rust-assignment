mod plane;

use plane::engine::Plane;
use plane::seat::BookingError;

fn main() {
    let mut plane = Plane::new(5);

    plane.book_seat(1, "Alice".to_string()).unwrap();
    plane.book_seat(2, "Bob".to_string()).unwrap();

    println!("\n--- Current Seats ---");
    plane.display_seats();

    match plane.book_seat(2, "Charlie".to_string()) {
        Err(BookingError::SeatAlreadyTaken) => println!("Seat already taken!"),
        _ => {}
    }

    plane.cancel_booking(1).unwrap();

    println!("\n--- After Cancellation ---");
    plane.display_seats();

    println!("\n--- Trying Overbooking ---");
    plane.try_overbook();
}
