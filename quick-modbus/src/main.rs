// QUICK-MODBUS
// Author: Mario Baldini <mariobaldini@gmail.com>
// Description: Tool for simple interaction with modbus devices. 
//              Can be compiled and run in multiple targets (PC, Raspberry PI, Beaglebone, etc)

// Build requirements:
// sudo apt install linux-libc-dev librust-libc-dev libcxxtools-dev glibc-source libc++-dev clang autoconf curl libtool librust-libc+rustc-dep-of-std-dev librust-libc+rustc-std-workspace-core-dev gnu-standards gettext 
// cargo install cross
// cross build --release --target armv7-unknown-linux-gnueabihf


extern crate libmodbus_rs;
extern crate failure;

use std::env;
use std::process;
use libmodbus_rs::{Modbus, ModbusClient, ModbusRTU};
use failure::Error;

pub fn main() -> Result<(), Error> {

  const VERSION: &'static str = env!("CARGO_PKG_VERSION");

  println!("----------- QUICK-MODBUS  ver. {:} ----------", VERSION);
  println!("https://github.com/mariobaldini/sensors-misc/tree/master/quick-modbus");
  println!("");

  let args: Vec<String> = env::args().collect();
  let filename = &args[0];
  
  if args.len() != 10 && args.len() != 11 {
    println!("Error! Missing arguments! Received: {:?} \nUsage example:", args);
    println!("{:} LOG_LEVEL PORT_NAME    BAUD DATABIT PARITY STOP_BIT DEVICE_ADDR    COMMAND        REGISTER [VALUE]", filename);
    println!("{:} verbose   /dev/ttyUSB0 9600 8       N      1        1              write_register 1        1"      , filename);
    println!("{:} verbose   /dev/ttyUSB0 9600 8       N      1        1              write_coil     1        1"      , filename);
    println!("{:} verbose   /dev/ttyUSB0 9600 8       N      1        1              read_register  1         "      , filename);
    println!("{:} verbose   /dev/ttyUSB0 9600 8       N      1        1              read_coil      1         "      , filename);
    process::exit(1);
  }

  let log_level      =  args[1].as_str();
  let port_name      = &args[2];
  let baud           =  args[3].parse::<i32>().unwrap();
  let data_bit       =  args[4].parse::<i32>().unwrap();
  let parity         =  args[5].parse::<char>().unwrap();
  let stop_bit       =  args[6].parse::<i32>().unwrap();
  let device_address =  args[7].parse::<u8>().unwrap();
  let cmd            =  args[8].as_str();
  let register       =  args[9].parse::<u16>().unwrap();
  let mut value: u16 = 0;
  if args.len() == 11 { 
    value           =   args[10].parse::<u16>().unwrap();
  }

  let mut modbus = Modbus::new_rtu(port_name, baud, parity, data_bit, stop_bit).unwrap();
  modbus.set_slave(device_address)?;
  
  if log_level == "verbose" {
    modbus.set_debug(true)?;
  }
  

  modbus.connect()?;

  match cmd { 
    "read_register" => {
      // Function code: 0x03   Read single  Analog Output Holding Register
      let mut dest = vec![0u16; 1];
      assert!(modbus.read_registers(register, 1, &mut dest).is_ok());

      if log_level == "verbose" {
        println!("\nReading register: {:?}", register);
        println!("Value read:{:?}", dest);  
      } else {
        println!("{:?}", dest);  
      }
      
      
    }

    "write_register" => {
      // Function code: 0x06   Write single  Analog Output Holding Register
      println!("\nWriting value: {:?} to register: {:?}", value, register);
      assert!(modbus.write_register(register, value).is_ok());
    }

    "read_coil" => {
      // Function code: 0x01   Write single   Discrete Output Coil
      let mut dest = vec![0u8; 1];
      assert!(modbus.read_bits(register, 1, &mut dest).is_ok());

      if log_level == "verbose" {
        println!("Value read:{:?}", dest);
      } else {
        println!("{:?}", dest);  
      }
      
      
    }

    "write_coil" => {
      // Function code: 0x05   Write single   Discrete Output Coil
      let value_bool: bool = value%2 == 1;
      assert!(modbus.write_bit(register, value_bool).is_ok());
    }

    _ => {
      println!("Error! Unknown command: {:?}", cmd);
      process::exit(1);
    }
  }

  Ok(())    
}
