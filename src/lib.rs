extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate failure;

use std::net::IpAddr;
use failure::Error;
use reqwest::Client;

#[derive(Debug)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(Clone, PartialEq, Eq)]
pub struct GeoInfo {
    pub ip_address: String,
    pub country_code: String,
    pub country_name: String,
    pub region_name: String,
    pub city_name: String,
    pub zip_code: String,
    pub latitude: String,
    pub longitude: String,
    pub time_zone: String,
}

/// Queries api for geolocation info.
/// 'api_key' The api key provided from ipinfodb, needed for making requests against api.
/// 'ip' The ip to query geolocation for, optional, if left out, the api will query current's ip geolocation.
pub fn query(api_key: &str, ip: Option<IpAddr>) -> Result<GeoInfo, Error> {
    let client = Client::new();

    let mut request = client.get("http://api.ipinfodb.com/v3/ip-city/");

    request.query(&[("key", api_key), ("format", "json")]);

    if let Some(addr) = ip {
        request.query(&[("ip", addr)]);
    }

    let geo_info: GeoInfo = request.send()?.json()?;

    Ok(geo_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ipaddr = "8.8.8.8".parse().unwrap();
        let queried_geo_info = query("YOUR_API_KEY", Some(ipaddr)).unwrap();

        let geo_info = GeoInfo {
            ip_address: "8.8.8.8".to_string(),
            country_code: "US".to_string(),
            country_name: "United States".to_string(),
            region_name: "California".to_string(),
            city_name: "Mountain View".to_string(),
            zip_code: "94043".to_string(),
            latitude: "37.406".to_string(),
            longitude: "-122.079".to_string(),
            time_zone: "-07:00".to_string(),
        };

        assert_eq!(geo_info, queried_geo_info);
    }
}
