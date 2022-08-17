use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use chrono::{Datelike, DateTime, Duration, FixedOffset};
use chrono::NaiveDateTime;
use chrono::offset::Utc;
use icalendar::Calendar;
use icalendar::Component;
use icalendar::Event;
use rocket::fs::NamedFile;
use rocket::response::status::NotFound;
use serde::Deserialize;
use rocket_dyn_templates::{Template, context};

#[macro_use] extern crate rocket;

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct CircuitType {
    #[allow(non_camel_case_types)]
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
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
struct RaceListType {
    Races: Vec<RaceType>,
}

#[tokio::main]
async fn fetchAndSaveRaces() -> Result<(), Box<dyn std::error::Error>> {
    // Fetch race data
    let data = reqwest::get(format!("http://ergast.com/api/f1/{}.json", Utc::now().year()))
        .await?
        .json::<serde_json::Value>()
        .await?;

    // Store races in JSON data as RaceListType
    let races: RaceListType = serde_json::from_value(data["MRData"]["RaceTable"].clone()).unwrap();

    // Create a empty calendar
    #[allow(non_snake_case)]
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

    // Generate filename on the form f1-races-{day}.{month}.{year}
    let fileNameWithTimestamp = format!("f1-races-{day}.{month}.{year}.ics",
                           day=Utc::now().day(),
                           month=Utc::now().month(),
                           year=Utc::now().year());


    let mut f = File::create(format!("./static/f1-races.ics")).expect("Unable to create file");
    f.write_all(data.as_bytes()).expect("Unable to write data");

    Ok(())
}

#[get("/")]
fn index() -> Template {
    Template::render("races", context! { name: "Andreas" })
}

// Download file if it exists in /static
#[get("/<file..>")]
async fn files(file: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("static/").join(file);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index, files])
        .ignite().await?
        .launch().await?;

    Ok(())
}