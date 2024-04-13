#[allow(unused_imports)]
use std::str;

use serde_derive::Deserialize;
use serde_derive::Serialize;
use geometry_rs::{{Point, Polygon}};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    #[serde(rename = "type")]
    pub type_field: String,
    pub features: Vec<Feature>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feature {
    #[serde(rename = "type")]
    pub type_field: String,
    pub geometry: Geometry,  // All data is "MultiPolygon"
    pub properties: Properties,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Geometry {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<Vec<Vec<Vec<f64>>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    #[serde(rename = "iso_a2")]
    pub iso_a2: String,
    #[serde(rename = "left_handed")]
    pub left_handed: bool,
}

pub fn load_geojson_file() -> Vec<u8> {
    include_bytes!("./data.geojson").to_vec()
}


pub fn load_geojson() -> Root {
    let data = load_geojson_file();
    let root: Root = serde_json::from_slice(&data).unwrap();
    root
}


pub struct CountryGeocoderItem {
    pub properties: Properties,
    pub polygons: Vec<geometry_rs::Polygon>
}

pub struct CountryGeocoder {
    pub items: Vec<CountryGeocoderItem>
}

impl CountryGeocoder {
    pub fn new() -> CountryGeocoder {
        println!("Loading geojson");
        let root = load_geojson();
        let mut items = Vec::new();
        println!("Root features len: {}", root.features.len());

        let mut feature_counter = 0;
        
        for feature in root.features {
            println!("Feature counter: {}", feature_counter);
            feature_counter += 1;

            let mut polygons: Vec<Polygon> = Vec::new();
            println!("Coords len: {}", feature.geometry.coordinates.len());

            for coordinates in feature.geometry.coordinates {
                let mut exterior: Vec<Point> = vec![];
                let mut interior: Vec<Vec<Point>> = vec![];

                for (i, v) in coordinates.iter().enumerate(){
                    if i == 0 {
                        for point in v {
                            exterior.push(Point{x: point[0], y: point[1]});
                        }
                    } else {
                        let mut points = Vec::new();
                        for point in v {
                            points.push(Point{x: point[0], y: point[1]});
                        }
                        interior.push(points);
                    }
                }


                println!("Exterior len: {}", exterior.len());
                println!("Interior len: {}", interior.len());
                if exterior.len() == 0 {
                    println!("bug data from geojson:{:?}", feature.properties);
                    continue;
                }
                let geopoly = geometry_rs::Polygon::new(exterior, interior);
                polygons.push(geopoly);
            }
            items.push(CountryGeocoderItem {
                properties: feature.properties,
                polygons: polygons
            });
        }

        println!("Items len: {}", items.len());

        CountryGeocoder {
            items
        }
    }

    pub fn geocode(&self, point: Point) -> Option<&Properties> {
        for item in &self.items {
            // Print len polygon
            println!("Polygon len: {}", item.polygons.len());

            for polygon in &item.polygons {
                if polygon.contains_point(point) {
                    return Some(&item.properties);
                }
            }
        }
        None
    }
}


#[cfg(test)]
mod tests {
    use super::{{CountryGeocoder, load_geojson}};
    use geometry_rs::Point;

    #[test]
    fn test_load_geojson() {
        let root = load_geojson();
        assert_eq!(root.type_field, "FeatureCollection");
        assert_eq!(root.features.len(), 177);
    }

    #[test]
    fn test_country_geocoder() {
        let geocoder = CountryGeocoder::new();
        let point = Point{x: 116.0, y: 39.0};
        let properties = geocoder.geocode(point).unwrap();
        assert_eq!(properties.iso_a2, "CN");
    }
}