use serde::Serialize;
use serde::Deserialize;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Trip {
    id: u32,
    shipper_name: String,
    origin_city: String,
    origin_state: State,
    receiver_name: String,
    destination_city: String,
    destination_state: State,
    cargo_type: String,
    payment_amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
enum State{
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    Ohio,
    Oklahoma,
    Oregon,
    Pennsylvania,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    Utah,
    Vermont,
    Virginia,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

impl State {
    pub fn abbreviation(&self) -> &'static str {
        match self {
            State::Alabama => "AL",
            State::Alaska => "AK",
            State::Arizona => "AZ",
            State::Arkansas => "AR",
            State::California => "CA",
            State::Colorado => "CO",
            State::Connecticut => "CT",
            State::Delaware => "DE",
            State::Florida => "FL",
            State::Georgia => "GA",
            State::Hawaii => "HI",
            State::Idaho => "ID",
            State::Illinois => "IL",
            State::Indiana => "IN",
            State::Iowa => "IA",
            State::Kansas => "KS",
            State::Kentucky => "KY",
            State::Louisiana => "LA",
            State::Maine => "ME",
            State::Maryland => "MD",
            State::Massachusetts => "MA",
            State::Michigan => "MI",
            State::Minnesota => "MN",
            State::Mississippi => "MS",
            State::Missouri => "MO",
            State::Montana => "MT",
            State::Nebraska => "NE",
            State::Nevada => "NV",
            State::NewHampshire => "NH",
            State::NewJersey => "NJ",
            State::NewMexico => "NM",
            State::NewYork => "NY",
            State::NorthCarolina => "NC",
            State::NorthDakota => "ND",
            State::Ohio => "OH",
            State::Oklahoma => "OK",
            State::Oregon => "OR",
            State::Pennsylvania => "PA",
            State::RhodeIsland => "RI",
            State::SouthCarolina => "SC",
            State::SouthDakota => "SD",
            State::Tennessee => "TN",
            State::Texas => "TX",
            State::Utah => "UT",
            State::Vermont => "VT",
            State::Virginia => "VA",
            State::Washington => "WA",
            State::WestVirginia => "WV",
            State::Wisconsin => "WI",
            State::Wyoming => "WY"
        }
    }
}




fn main() -> Result<(), Box<dyn Error>> {
    use std::fs;

    let my_first_trip = Trip {
        id: 20008000,
        shipper_name: String::from("Majestic Loads"),
        origin_city: String::from("Oklahoma City"),
        origin_state: State::Oklahoma,
        receiver_name: String::from("FedEx"),
        destination_city: String::from("Great Falls"),
        destination_state: State::Montana,
        cargo_type: String::from("Confetti"),
        payment_amount: 162_200,
    };

    println!("Trip: \n");
    println!("Origin: {}, {}", my_first_trip.origin_city, my_first_trip.origin_state.abbreviation());
    println!("Destination: {}, {}", my_first_trip.destination_city, my_first_trip.destination_state.abbreviation());
    println!("Shipper: {}", my_first_trip.shipper_name);
    println!("Receiver: {}", my_first_trip.receiver_name);


    let json = serde_json::to_string_pretty(&my_first_trip)?;
    fs::write("shipments.json", json)?;
    Ok(())
}

fn save(trips: &Vec<Trip>) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(trips)?;
    Ok(())
}
