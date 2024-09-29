/*
 enums can also behave like structs. 
 This means that each variant can hold data fields,
  similar to a struct
*/
#[derive(Debug)]
enum Vehicle {
    Car { speed: f64 },           // Car with speed in km/h
    Bike { speed: f64 },          // Bike with speed in km/h
    Airplane { speed: f64, altitude: f64 }, // Airplane with speed in km/h and altitude in meters
}

fn max_speed(vehicle: &Vehicle) -> f64 {
    match vehicle {
        Vehicle::Car { speed } => speed *1.0,
        Vehicle::Bike { speed } => *speed,
        Vehicle::Airplane { speed, altitude } => {
            // Airplane speed decreases slightly with higher altitude
            if *altitude > 10000.0 {
                speed * 0.9
            } else {
                *speed
            }
        }
    }
}

fn main() {
    let car = Vehicle::Car { speed: 180.0 };
    let bike = Vehicle::Bike { speed: 40.0 };
    let airplane = Vehicle::Airplane { speed: 900.0, altitude: 12000.0 };

    println!("Car max speed: {} km/h", max_speed(&car));
    println!("Bike max speed: {} km/h", max_speed(&bike));
    println!("Airplane max speed: {} km/h", max_speed(&airplane));
}