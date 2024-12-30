use std::{
    thread::sleep,
    time::Duration,
};
use heapless::String;
use esp_idf_sys as _;
use esp_idf_hal::{
    peripherals::Peripherals,
};
use esp_idf_svc::{
    wifi::EspWifi,
    nvs::EspDefaultNvsPartition,
    eventloop::EspSystemEventLoop,
};
use embedded_svc::wifi::{ClientConfiguration, Wifi, Configuration};


fn main(){
    esp_idf_sys::link_patches();
    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi_driver = EspWifi::new(
        peripherals.modem,
        sys_loop,
        Some(nvs)
    ).unwrap();

    let mut user_ssid = String::new();
    user_ssid.push_str("MYSSID").unwrap();
    let mut user_password = String::new();
    user_password.push_str("MYPASSWORD").unwrap();

    wifi_driver.set_configuration(&Configuration::Client(ClientConfiguration{
        ssid: user_ssid,
        password: user_password,
        ..Default::default()
    })).unwrap();

    wifi_driver.start().unwrap();
    wifi_driver.connect().unwrap();
    while !wifi_driver.is_connected().unwrap(){
        let config = wifi_driver.get_configuration().unwrap();
        println!("Waiting for station {:?}", config);
    }
    println!("WiFi Connected");
    // loop{
    //     println!("IP info: {:?}", wifi_driver.sta_netif().get_ip_info().unwrap());
    //     sleep(Duration::new(10,0));
    // }

    println!("Retrieving ISS Position");
    let result = get_iss_position();

    println!("ISS POSITION:\n LAT: {}\nLON: {}",result.0,result.1);

}

pub fn get_iss_position() -> Result<(f64, f64, f64, f64), Box<dyn std::error::Error>> {
    // Pulled from my other rsISS project: https://github.com/donaldcampbelljr/rsISS
    let mut res = reqwest::blocking::get("https://api.wheretheiss.at/v1/satellites/25544")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    let json: Value = match serde_json::from_str(&body) {
        Ok(json) => json,
        Err(err) => return Err(Box::new(err)),
    };

    let latitude: f64 = json["latitude"].as_f64().expect("Desire a number");
    let longitude: f64 = json["longitude"].as_f64().expect("Desire a number");
    let altitude: f64 = json["altitude"].as_f64().expect("Desire a number");
    let timestamp: f64 = json["timestamp"].as_f64().expect("Desire a number");

    Ok((latitude, longitude, altitude, timestamp))
}