extern crate windows_service;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use windows_service::{
    define_windows_service, service::ServiceAccess, service_dispatcher, 
    service_manager::{ServiceManager, ServiceManagerAccess},
};

define_windows_service!(ffi_service_main, my_service_main);

fn service_manager(access: ServiceManagerAccess) -> windows_service::Result<ServiceManager> {
    ServiceManager::local_computer(None::<&str>, access)
}

// Main service logic
fn my_service_main(_arguments: Vec<OsString>) {
    let output = Command::new("whoami").output().expect("failed to execute process");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("C:\\\\PENTEST_SERVICE.txt")
        .expect("unable to create or open file");

    file.write_all(b"PENTEST SERVICE RAN AS: ").expect("unabled to write data");
    file.write_all(&output.stdout).expect("unable to write data");

    stop_service().expect("Unable to stop service");
}

// Function to stop the service
fn stop_service() -> windows_service::Result<()> {
    let manager = service_manager(ServiceManagerAccess::CONNECT)?;
    let service = manager.open_service(
        "PENTESTSERVICE", 
        ServiceAccess::STOP | ServiceAccess::QUERY_STATUS
    )?;

    match service.stop() {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

// Main function
fn main() -> windows_service::Result<()> {
    let args: Vec<OsString> = std::env::args_os().collect();

    service_dispatcher::start("PENTESTSERVICE", ffi_service_main)?;

    // Print current service status if arguments were supplied
    if args.len() > 1 {
        let _ = print_service_status();
    }

    Ok(())
}

// Print service status
fn print_service_status() -> windows_service::Result<()> {
    let manager = service_manager(ServiceManagerAccess::CONNECT)?;
    let service = manager.open_service(
        "PENTESTSERVICE", 
        ServiceAccess::QUERY_CONFIG | ServiceAccess::QUERY_STATUS
    )?;
    let service_status = service.query_status()?;
    let service_config = service.query_config()?;

    println!("Service status: {:?}", service_status);
    println!("Service config: {:?}", service_config);

    Ok(())
}
