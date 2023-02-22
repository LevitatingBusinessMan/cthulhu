use std::vec;

use crate::commands::*;

pub struct Weather;

impl CommandDetails for Weather {
	const ARITY: Arity = Arity::Minimum(1);
	const NAME: &'static str = "weather";
	const USAGE: &'static str = "<location>";
	const DESCRIPTION: &'static str = "Get the weather for a location using openweather";
	const DEPENDS: LazyCell<Vec<&'static str>> = LazyCell::new(|| vec!["openweather"]);
}

lazy_static! {
	static ref key: String = crate::config::API_KEYS.get("openweather").unwrap().clone().try_into().unwrap();
}

impl CommandMethods for Weather {
	fn run(&self, user: User, arguments: Vec<String>, _target: &String) -> String {
		let loc = openweather::LocationSpecifier::CityAndCountryName {city: arguments.join(" "), country: "".to_string()};
		
		match openweather::get_current_weather(&loc, &key, &openweather::Settings{unit: Some(openweather::Unit::Metric), lang: None}) {
			Ok(weather) => {
				let header = bold!(format!("Weather in {}, {}", weather.name, weather.sys.country));
				let description = format!("Description: {}",
					match weather.weather.get(0) {
						Some(weather) => weather.description.clone(),
						None => "unknown".to_owned()
					}
				);
				return format!("\
					{}\n\
					{}\n\
					Temp: {}°C, Wind: {}m/s, Humidity: {}%\
					"
					, header, description, weather.main.temp, weather.wind.speed, weather.main.humidity)
			},
			Err(err) => {
				eprintln!("{}", err);
				return "Some error occured".to_string();
			}
		}

	}
	crate::command_methods!();
}
