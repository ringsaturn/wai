#![allow(clippy::needless_return)]

use country_geocoder::CountryGeocoder;
use geo::{Coord, Point};
use lazy_static::lazy_static;
use reverse_geocoder::ReverseGeocoder;
use tzf_rs::DefaultFinder;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::default();
    static ref COUNTRY_GEOCODER: CountryGeocoder = CountryGeocoder::new();
    static ref REVERSE_GEOCODER: ReverseGeocoder = ReverseGeocoder::new();
}

// get timezone name
pub fn get_tz(lng: f64, lat: f64) -> String {
    FINDER.get_tz_name(lng, lat).to_string()
}

// get timezone names
pub fn get_tzs(lng: f64, lat: f64) -> Vec<String> {
    FINDER.get_tz_names(lng, lat).iter().map(|x| x.to_string()).collect()
}

// get country code
pub fn get_country(lng: f64, lat: f64) -> String {
    if let Some(res) = COUNTRY_GEOCODER.iso_a2(Point(Coord { x: lng, y: lat })) {
        return res.to_string()
    }
    "".to_string()
}
