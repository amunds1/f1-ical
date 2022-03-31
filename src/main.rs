use chrono::offset::Utc;
use chrono::{Datelike, DateTime, Duration, FixedOffset};

use chrono::NaiveDateTime;
use icalendar::Calendar;
use icalendar::Component;
use icalendar::Event;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
struct CircuitType {
	circuitId: String,
	url: String,
	circuitName: String,
	Location: LocationType,
}

#[derive(Deserialize, Debug)]
struct LocationType {
	lat: String,
	long: String,
	locality: String,
	country: String,
}

#[derive(Deserialize, Debug)]
struct RaceType {
	season: String,
	round: String,
	url: String,
	raceName: String,
	Circuit: CircuitType,
	date: String,
	time: String,
}

#[derive(Deserialize, Debug)]
struct RaceListType {
	Races: Vec<RaceType>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Fetch race data
	let data = reqwest::get(format!("http://ergast.com/api/f1/{}.json", Utc::now().year()))
		.await?
		.json::<serde_json::Value>()
		.await?;

	// Store races in JSON data as RaceListType
	let races: RaceListType = serde_json::from_value(data["MRData"]["RaceTable"].clone()).unwrap();

	// Create a empty calendar
	let mut raceCalendar = Calendar::new();

	// Iterate over races
	for race in races.Races {
		// Convert race date and time into DateTime<Utc>
		let ndt = NaiveDateTime::parse_from_str(format!("{} {}", race.date, race.time).as_str(), "%Y-%m-%d %H:%M:%S%Z");
		let dt = DateTime::<Utc>::from_utc(ndt.unwrap(), Utc);

		// Create race event
		let event = Event::new()
			.starts(dt)
			.ends(dt + Duration::hours(2))
			.summary(&race.raceName.to_string())
			.location(
				&format!(
					"{}, {}",
					race.Circuit.Location.locality, race.Circuit.Location.country
				)
				.to_string(),
			)
			.done();

		// Push event to calendar
		raceCalendar.push(event);
	}

	// Write calendar to a .ics file
	let data = raceCalendar.to_string();
	let mut f = File::create("icalendar.ics").expect("Unable to create file");
	f.write_all(data.as_bytes()).expect("Unable to write data");

	Ok(())
}
