use std::collections::HashMap;
use serialport::available_ports;

pub fn scan() {
    match available_ports() {
        Ok(ports) => {
            for port in ports {
                println!("Port: {}", port.port_name);

                match port.port_type {
                    serialport::SerialPortType::UsbPort(info) => {
                        println!("  VID: {:04x}", info.vid);
                        println!("  PID: {:04x}", info.pid);
                        println!("  Manufacturer: {:?}", info.manufacturer);
                        println!("  Product: {:?}", info.product);
                        println!("  Serial: {:?}", info.serial_number);
                    }
                    _ => println!("  Non-USB port"),
                }
            }
        }
        Err(e) => println!("Error listing ports: {:?}", e),
    }
}

//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
