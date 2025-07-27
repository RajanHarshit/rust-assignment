use std::panic;

/// Custom seat allocation error
#[derive(Debug)]
enum BookingError {
    SeatUnavailable,
    InvalidSeat,
}

/// Represents a seat with a passenger's name
#[derive(Debug, Clone)]
struct Seat {
    passenger: String,
}

/// A plane with seats: Vec<Option<Seat>>
#[derive(Clone)]
struct Plane {
    seats: Vec<Option<Seat>>, // None = available, Some(seat) = booked
}

impl Plane {
    fn new(seat_count: usize) -> Self {
        Plane {
            seats: vec![None; seat_count],
        }
    }

    /// Try to book a seat by index
    fn book_seat(&mut self, index: usize, name: &str) -> Result<(), BookingError> {
        match self.seats.get_mut(index) {
            Some(slot) => {
                match slot {
                    Some(_) => Err(BookingError::SeatUnavailable),
                    None => {
                        *slot = Some(Seat {
                            passenger: name.to_string(),
                        });
                        println!("✅ Seat {} booked for {}", index, name);
                        Ok(())
                    }
                }
            }
            None => Err(BookingError::InvalidSeat),
        }
    }

    /// Free a seat
    fn free_seat(&mut self, index: usize) -> Result<(), BookingError> {
        match self.seats.get_mut(index) {
            Some(slot) => {
                if slot.is_some() {
                    println!("🔓 Seat {} is now free.", index);
                    *slot = None;
                    Ok(())
                } else {
                    Err(BookingError::SeatUnavailable)
                }
            }
            None => Err(BookingError::InvalidSeat),
        }
    }

    /// Show current seat map
    fn show_seats(&self) {
        println!("\n📋 Seat Map:");
        for (i, seat) in self.seats.iter().enumerate() {
            match seat {
                Some(seat) => println!("Seat {}: Occupied by {}", i, seat.passenger),
                None => println!("Seat {}: Available", i),
            }
        }
    }

    /// Book the first available seat (pattern guard)
    fn auto_book(&mut self, name: &str) -> Result<usize, BookingError> {
        for (i, slot) in self.seats.iter_mut().enumerate() {
            if let None = slot {
                *slot = Some(Seat {
                    passenger: name.to_string(),
                });
                println!("✅ Auto-booked seat {} for {}", i, name);
                return Ok(i);
            }
        }

        // No seat found — overbooking attempt
        panic!("🚨 Overbooking detected! No available seats left.");
    }
}

// Panic recovery
fn safe_booking<F: FnOnce() -> R + panic::UnwindSafe, R>(operation: F) -> Option<R> {
    match panic::catch_unwind(operation) {
        Ok(result) => Some(result),
        Err(_) => {
            println!("⚠️ Recovered from panic (overbooking).");
            None
        }
    }
}

// ---------- MAIN ----------
fn main() {
    let mut plane = Plane::new(5); // plane with 5 seats

    // Manual booking
    plane.book_seat(0, "Alice").unwrap();
    plane.book_seat(1, "Bob").unwrap();

    // Show state
    plane.show_seats();

    // Free a seat
    plane.free_seat(0).unwrap();

    // Pattern guard: Auto-book first available
    plane.auto_book("Charlie").unwrap();
    plane.auto_book("Dana").unwrap();
    plane.auto_book("Eve").unwrap();
    plane.auto_book("Frank").unwrap();

    safe_booking(|| {
        let mut cloned_plane = plane.clone();
        cloned_plane.auto_book("Grace").unwrap();
    });

    // Final state
    plane.show_seats();
}
