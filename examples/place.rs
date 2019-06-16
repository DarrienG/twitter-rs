// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

extern crate egg_mode;

mod common;

use common::tokio::runtime::current_thread::block_on_all;

use egg_mode::place::{PlaceType, Point};

fn main() {
    let config = common::Config::load();

    let result = block_on_all(
        egg_mode::place::search_query("columbia")
            .granularity(PlaceType::Admin)
            .max_results(10)
            .call(&config.token),
    )
    .unwrap();

    println!(
        "{} results for \"columbia\", administrative regions or larger:",
        result.results.len()
    );

    for place in &result.results {
        println!("{}", place.full_name);
    }
    println!("");

    let result = block_on_all(
        egg_mode::place::reverse_geocode(Point::new(-0.1275, 51.507222))
            .granularity(PlaceType::City)
            .call(&config.token),
    )
    .unwrap();

    println!(
        "{} results for reverse-geocoding {}, {}:",
        result.results.len(),
        -0.1275,
        51.507222,
    );

    for place in &result.results {
        println!("{}", place.full_name);
    }
}
