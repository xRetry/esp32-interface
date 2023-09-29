use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::{sync::{Mutex, Arc}, time::Duration};
use embedded_svc::http::{Method, server::HandlerResult};
use esp_idf_svc::{eventloop::EspSystemEventLoop, http::server::{EspHttpServer, Configuration}};
use esp_idf_hal::prelude::*;
use anyhow::Result;

mod wifi;
mod hardware;

use hardware::Driver;

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();
    let peripherals = Peripherals::take().unwrap();

    let sysloop = EspSystemEventLoop::take()?;
    let _ = wifi::wifi(
        "test",
        "abc",
        peripherals.modem,
        sysloop,
    )?;

    let driver = Arc::new(Mutex::new(Driver::new(peripherals.pins)));

    let config = Configuration::default();
    let mut server = EspHttpServer::new(&config)?;
    let d = driver.clone();
    server.fn_handler("/", Method::Get, move |request| {
        d.lock().unwrap().digital_read(18);
        return HandlerResult::Ok(());
    })?;
    
    //let driver = driver.lock().unwrap().set_digital_input(18);

    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}
