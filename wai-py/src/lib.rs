#![allow(clippy::needless_return)]

use country_geocoder::CountryGeocoder;
use geo::{Coord, Point};
use lazy_static::lazy_static;
use pyo3::prelude::*;
use reverse_geocoder::ReverseGeocoder;
use tzf_rs::DefaultFinder;

lazy_static! {
    static ref FINDER: DefaultFinder = DefaultFinder::default();
    static ref COUNTRY_GEOCODER: CountryGeocoder = CountryGeocoder::new();
    static ref REVERSE_GEOCODER: ReverseGeocoder = ReverseGeocoder::new();
}

#[pyfunction]
pub fn get_tz(lng: f64, lat: f64) -> PyResult<String> {
    Ok(FINDER.get_tz_name(lng, lat).to_string())
}

#[pyfunction]
pub fn get_tzs(_py: Python, lng: f64, lat: f64) -> PyResult<Vec<&str>> {
    Ok(FINDER.get_tz_names(lng, lat))
}

#[pyfunction]
pub fn get_country(_py: Python, lng: f64, lat: f64) -> PyResult<String> {
    if let Some(res) = COUNTRY_GEOCODER.iso_a2(Point(Coord { x: lng, y: lat })) {
        Ok(res.to_string())
    } else {
        Ok("".to_string())
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct CityRecord {
    #[pyo3(get)]
    pub distance: f64,
    #[pyo3(get)]
    pub lat: f64,
    #[pyo3(get)]
    pub lng: f64,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub admin1: String,
    #[pyo3(get)]
    pub admin2: String,
    #[pyo3(get)]
    pub cc: String,
}

#[pymethods]
impl CityRecord {
    fn __repr__(slf: PyRef<'_, Self>) -> PyResult<String> {
        return Ok(format!(
            "City({distance}): cc={cc}, name={name}, lng={lng}, lat={lat}, admin1={admin1}, admin2={admin2}",
            distance=slf.distance,
            cc=slf.cc,
            name=slf.name,
            lng=slf.lng,
            lat=slf.lat,
            admin1=slf.admin1,
            admin2=slf.admin2,
        ));
    }
}

#[pyfunction]
pub fn get_city(_py: Python, lng: f64, lat: f64) -> PyResult<Option<CityRecord>> {
    let result = REVERSE_GEOCODER.search((lat, lng));
    Ok(Some(CityRecord {
        distance: result.distance,
        lat: result.record.lat,
        lng: result.record.lon,
        name: result.record.name.clone(),
        admin1: result.record.admin1.clone(),
        admin2: result.record.admin2.clone(),
        cc: result.record.cc.clone(),
    }))
}

#[pymodule]
fn wai(_py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_tz, m)?)?;
    m.add_function(wrap_pyfunction!(get_tzs, m)?)?;
    m.add_function(wrap_pyfunction!(get_country, m)?)?;
    m.add_function(wrap_pyfunction!(get_city, m)?)?;
    Ok(())
}
