use serde::Serialize;
use serde::Deserialize;
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
struct Trip {
    id: u64,
    shipper_name: String,
    origin_city: String,
    origin_state: State,
    receiver_name: String,
    destination_city: String,
    destination_state: State,
    cargo_type: String,
    payment_amount: u32,
}

impl Default for Trip {
    fn default() -> Self {
        Self {
            id: 0,
            shipper_name: String::new(),
            origin_city: String::new(),
            origin_state: State::NewJersey,
            receiver_name: String::new(),
            destination_city: String::new(),
            destination_state: State::Pennsylvania,
            cargo_type: String::new(),
            payment_amount: 0
        }
    }
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

    // const FIRST_ID: u64 = 20008000;

    // let mut current_id = FIRST_ID;

    // let mut trip_log = Vec::new();
    // let trip = Trip {
    //     id: current_id,
    //     shipper_name: String::from("Majestic Loads"),
    //     origin_city: String::from("Oklahoma City"),
    //     origin_state: State::Oklahoma,
    //     receiver_name: String::from("FedEx"),
    //     destination_city: String::from("Great Falls"),
    //     destination_state: State::Montana,
    //     cargo_type: String::from("Confetti"),
    //     payment_amount: 162_200,
    // };
    // current_id += 1;

    // trip_log.push(trip);

    // let trip = Trip {
    //     id: current_id,
    //     shipper_name: String::from("Grain Campaign"),
    //     origin_city: String::from("Tulsa"),
    //     origin_state: State::Oklahoma,
    //     receiver_name: String::from("Lt. Mills"),
    //     destination_city: String::from("Barstow"),
    //     destination_state: State::California,
    //     cargo_type: String::from("Grain"),

    // for trip in &trip_log {
    //     println!("Trip: \n");
    //     println!("Origin: {}, {}", trip.origin_city, trip.origin_state.abbreviation());
    //     println!("Destination: {}, {}", trip.destination_city, trip.destination_state.abbreviation());
    //     println!("Shipper: {}", trip.shipper_name);
    //     println!("Receiver: {}", trip.receiver_name);
    // }

    // save(&trip_log)?;

    let mut trip_log = load_or_default();

    let trip = Trip {
        id: next_id(&trip_log),
        shipper_name: String::from("Gemini Labs"),
        origin_city: String::from("Pittsburgh"),
        origin_state: State::Pennsylvania,
        receiver_name: String::from("ClosedAI"),
        destination_city: String::from("Orlando"),
        destination_state: State::Florida,
        cargo_type: String::from("Liquid Electricity"),
        payment_amount: 32_600        
    };

    trip_log.push(trip);

    for trip in &trip_log {
        println!("Trip: \n");
        println!("Origin: {}, {}", trip.origin_city, trip.origin_state.abbreviation());
        println!("Destination: {}, {}", trip.destination_city, trip.destination_state.abbreviation());
        println!("Shipper: {}", trip.shipper_name);
        println!("Receiver: {}", trip.receiver_name);
    }

    save(&trip_log)?;

    Ok(())
}

fn save(trips: &[Trip]) -> Result<(), Box<dyn std::error::Error>> {
    
    let json = serde_json::to_string_pretty(trips)?;
    fs::write("trips.json", json)?;
    Ok(())
}


fn load() -> Result<Vec<Trip>, Box<dyn std::error::Error>> {
    let data = fs::read_to_string("trips.json")?;
    let trips = serde_json::from_str(&data)?;
    Ok(trips)
}

fn load_or_default() -> Vec<Trip> {
    match load() {
        Ok(data) => data,
        Err(_) => Vec::new(),
    }
}

fn next_id(trips: &[Trip]) -> u64 {
    trips
        .iter()
        .map(|t| t.id)
        .max()
        .unwrap_or(80006000) + 1
}