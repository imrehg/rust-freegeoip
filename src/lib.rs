#![crate_type = "lib"]
#![crate_name = "freegeoip"]

#![warn(non_camel_case_types,
        non_upper_case_globals,
        unused_qualifications)]

extern crate hyper;
extern crate rustc_serialize;

use std::io::Read;

use hyper::{ Client, Url };
use hyper::header::Connection;

use rustc_serialize::json;

/// This struct will contain the results of the query,
/// the results of this a query like this:
/// `curl https://freegeoip.net/json/8.8.8.8`
///
/// # Examples
///
/// ```
/// let ip = "8.8.8.8".to_string();
/// let geoip = freegeoip::query(&ip);
/// println!("{} is in the city of {}", ip, geoip.city);
/// ```
#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct GeoIPInfo {
    /// IP address, e.g. "8.8.8.8"
    pub ip: String,
    /// Two-letter country code, e.g. "US", might be empty
    pub country_code: String,
    /// Country name, e.g. "United States", might be empty
    pub country_name: String,
    /// Region code, e.g. "CA", might be empty
    pub region_code: String,
    /// Region name, e.g. "California", might be empty
    pub region_name: String,
    /// City name, e.g. "Montain View", might be empty
    pub city: String,
    /// Zip or postal code, e.g. "94040", might be empty
    pub zip_code: String,
    /// Time zone name as in the tz database, e.g. "America/Los_Angeles", might be empty
    pub time_zone: String,
    /// Geographic latitude, +/- is N/S, e.g. 37.3845,  might be empty
    pub latitude: f64,
    /// Geographic longitude, +/- is E/W, e.g. -122.0881,  might be empty
    pub longitude: f64,
    /// Metro code, used in the US only, e.g. 807, might be 0
    pub metro_code: i64,
}

/// Query GeoIP information for `target`.
///
///
/// # Arguments
///
/// *  `target` - can be an IP address (IPv4 or IPv6) or a domain name.
///
/// # Example
///
/// ```
/// let geoip = freegeoip::query("8.8.8.8");
/// ```
pub fn query(target: &str) -> GeoIPInfo {
    let api_base = "https://freegeoip.net/json/".to_string();
    let full_url = api_base + &target;
    let uri = Url::parse(&full_url).ok().expect("malformed url");

    let client = Client::new();
    let mut res = client.get(uri)
        .header(Connection::close())
        .send().unwrap();
    
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    let geo: GeoIPInfo = json::decode(&body).unwrap();    
    geo
}
