extern crate windows_service;
use std::ffi::OsString;
use windows_service::{service::{ServiceAccess, ServiceInfo, ServiceStartType, ServiceType},
                     service_manager::{ServiceManager, ServiceManagerAccess},
                     service_dispatcher, define_windows_service};
use std::process::Command;
use std::fs::File;
use std::io::Write;

define_windows_service!(ffi_service_main, my_service_main);

fn my_service_main(_arguments: Vec<OsString>) {
    let output = Command::new("whoami")
        .output()
        .expect("failed to execute process");

    let mut file = File::create("C:\\\\PENTEST_WHOAMI.txt")
        .expect("unable to create file");

    file.write_all(b"PENTEST SERVICE RAN AS: ")
        .expect("unabled to write data");

    file.write_all(&output.stdout)
        .expect("unable to write data");
    
    stop_service().expect("unable to stop service"); // Stop the service after execution
}

fn stop_service() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_access = ServiceAccess::STOP | ServiceAccess::QUERY_STATUS;
    let service = manager.open_service("PENTEST", service_access)?;

    match service.stop() {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn print_service_status() -> windows_service::Result<()> {
    let manager_access = ServiceManagerAccess::CONNECT;
    let manager = ServiceManager::local_computer(None::<&str>, manager_access)?;

    let service_access = ServiceAccess::QUERY_CONFIG | ServiceAccess::QUERY_STATUS;
    let service = manager.open_service("PENTEST", service_access)?;
    let service_status = service.query_status()?;
    let service_config = service.query_config()?;

    println!("Service status: {:?}", service_status);
    println!("Service config: {:?}", service_config);

    Ok(())
}

fn main() -> windows_service::Result<()> {
    let args: Vec<OsString> = std::env::args_os().collect();

    if args.len() == 1 {
        // No arguments were supplied
        service_dispatcher::start("PENTEST", ffi_service_main)?;
    } else {
        // Print current service status
        let _ = print_service_status();
    }

    Ok(())
}

