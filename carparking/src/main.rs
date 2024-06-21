use chrono::{DateTime, Utc};
use rand::Rng;
use std::{fmt, io};
use std::fmt::Debug;

#[derive(Debug, Clone)]
struct VehicleParking {
    token: String,
    vehicle_no: String,
    name: String,
    mobile: u32,
    address: String,
    hour: u8,
    tax_per_hour: u16,
    parking_tax: u32,
    entry_time: DateTime<Utc>,
}
struct ParkingReceipt {
    token: String,
    floor: u8,
    spot: u8,
    timestamp: DateTime<Utc>,
    vehicle_no: String,
    total_hours: u8,
    rate_per_hour: u16,
    total_parking_tax: u32,
}
#[derive(Debug)]
struct Building {
    floors: Vec<Vec<Option<VehicleParking>>>,
    floor_count: u8,
    spots_per_floor: u8,
}
impl Building {
    fn new(floor_count: u8, spots_per_floor: u8) -> Self {
        let floors = vec![vec![None; spots_per_floor as usize]; floor_count as usize];
        Building {
            floors,
            floor_count,
            spots_per_floor,
        }
    }
    fn park_vehicle(&mut self, vehicle: VehicleParking) -> Result<ParkingReceipt, String> {
        for (floor_index, floor) in self.floors.iter_mut().enumerate() {
            for (spot_index, spot) in floor.iter_mut().enumerate() {
                if spot.is_none() {
                    let recepit = ParkingReceipt {
                        token: vehicle.token.clone(),
                        floor: floor_index as u8,
                        spot: spot_index as u8,
                        timestamp: vehicle.entry_time,
                        vehicle_no: vehicle.vehicle_no.clone(),
                        total_hours: vehicle.hour,
                        rate_per_hour: 30,
                        total_parking_tax: u32::from(vehicle.hour) * 30,
                    };
                    *spot = Some(vehicle);
                    return Ok(recepit);
                }
            }
        }
        Err("No available parking spots".to_string())
    }
    fn remove_vehicle(&mut self, token: &str) -> Option<VehicleParking> {
        for floor in self.floors.iter_mut() {
            for spot in floor.iter_mut() {
                if let Some(vehicle) = spot {
                    if vehicle.token == token {
                        return spot.take();
                    }
                }
            }
        }
        None
    }
    fn get_available_spots(&self) -> u32 {
        self.floors
            .iter()
            .flatten()
            .filter(|spot| spot.is_none())
            .count() as u32
    }
}
impl VehicleParking {
    fn new(
        vehicle_no: String,
        name: String,
        mobile: u32,
        address: String,
        hour: u8,
        tax_per_hour: u16,
        parking_tax: u32,
    ) -> Self {
        VehicleParking {
            token: Self::generate_token(10),
            vehicle_no,
            name,
            mobile,
            address,
            hour,
            tax_per_hour,
            parking_tax,
            entry_time: Utc::now(),
        }
    }

    fn generate_token(length: u8) -> String {
        let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                          abcdefghijklmnopqrstuvwxyz\
                          0123456789";

        let mut rng = rand::thread_rng();
        (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }
    fn calculate_total_cost(&self) -> u32 {
        let base_cost = u32::from(self.hour) * u32::from(self.tax_per_hour);
        base_cost + self.parking_tax
    }
}

impl fmt::Display for ParkingReceipt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parking Receipt\n----------------\n")?;
        write!(f, "Token: {}\n", self.token)?;
        write!(f, "Floor: {}\n", self.floor)?;
        write!(f, "Spot: {}\n", self.spot)?;
        write!(f, "Timestamp: {}\n", self.timestamp)?;
        write!(f, "Vehicle Number: {}\n", self.vehicle_no)?;
        write!(f, "Total Hours: {}\n", self.total_hours)?;
        write!(f, "Rate per Hour: ${}\n", self.rate_per_hour)?;
        write!(f, "Total Parking Tax: ${}\n", self.total_parking_tax)
    }
}
fn main() {
    let mut building = Building::new(3, 3);
    let bmw = VehicleParking::new(
        String::from("ABC123"),
        String::from("John Doe"),
        1234567890,
        String::from("123 Main St"),
        2,
        150,
        50,
    );
    match building.park_vehicle(bmw) {
        Ok(receipt) => println!("Vehicle parked successfully:\n{}", receipt),
        Err(e) => println!("Error: {}", e),
    }
    println!("Available spots: {}", building.get_available_spots());
    let car1 = VehicleParking::new(
        String::from("ABC123"),
        String::from("John Doe"),
        1234567890,
        String::from("123 Main St"),
        2,
        150,
        50,
    );
    match building.park_vehicle(car1) {
        Ok(receipt) => println!("Vehicle parked successfully:\n{}", receipt),
        Err(e) => println!("Error: {}", e),
    }
    println!("Available spots: {}", building.get_available_spots());
    println!("if you want collect you car from our parking area . Please enter you parking token:");

    let mut remove_car=String::new();
    io::stdin().read_line(&mut remove_car).expect("Vehicle Token not found");
    let remove_car = remove_car.trim();
    match building.remove_vehicle(&remove_car) {
        Some(vehicle) => println!("Vehicle successfully collected by {},{:?}",vehicle.name,vehicle),
        None => println!("Vehicle not found"),
    }
    
}
