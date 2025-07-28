use std::panic::{self, AssertUnwindSafe};

use super::seat::{BookingError, Seat};

pub struct Plane {
    seats: Vec<Option<Seat>>,
}

impl Plane {
    pub fn new(total_seats: usize) -> Self {
        Plane {
            seats: vec![None; total_seats],
        }
    }

    pub fn book_seat(&mut self, seat_number: usize, name: String) -> Result<(), BookingError> {
        if seat_number >= self.seats.len() {
            return Err(BookingError::SeatInvalid);
        }

        if let Some(_) = self.seats[seat_number] {
            Err(BookingError::SeatAlreadyTaken)
        } else {
            self.seats[seat_number] = Some(Seat {
                seat_number,
                passenger_name: name,
            });
            Ok(())
        }
    }

    pub fn cancel_booking(&mut self, seat_number: usize) -> Result<(), BookingError> {
        if seat_number >= self.seats.len() {
            return Err(BookingError::SeatInvalid);
        }

        if let Some(_) = self.seats[seat_number] {
            self.seats[seat_number] = None;
            Ok(())
        } else {
            Err(BookingError::SeatInvalid)
        }
    }

    pub fn try_overbook(&mut self) {
        let result = panic::catch_unwind(AssertUnwindSafe(|| {
            for i in 0..=self.seats.len() {
                if let Err(err) = self.book_seat(i, format!("Passenger {}", i)) {
                    panic!("Overbooking attempt: {:?}", err);
                }
            }
        }));

        match result {
            Ok(_) => println!("All seats booked safely."),
            Err(e) => println!("Recovered from panic: {:?}", e),
        }
    }

    pub fn display_seats(&self) {
        for (i, seat) in self.seats.iter().enumerate() {
            match seat {
                Some(seat) => println!("Seat {}: Booked by {}", i, seat.passenger_name),
                None => println!("Seat {}: Available", i),
            }
        }
    }
}
