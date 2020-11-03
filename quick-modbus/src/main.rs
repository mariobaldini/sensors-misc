// QUICK-MODBUS
// Author: Mario Baldini <mariobaldini@gmail.com>
// Description: Tool for simple interaction with modbus devices. 
//              Can be compiled and run in multiple targets (PC, Raspberry PI, Beaglebone, etc)

// Build requirements:
// sudo apt install --install-recommends linux-libc-dev librust-libc-dev libcxxtools-dev glibc-source libc++-dev clang autoconf curl libtool librust-libc+rustc-dep-of-std-dev librust-libc+rustc-std-workspace-core-dev gnu-standards gettext 
// ? apt-get install autoconf build-essential curl clang-3.9 git-core libtool

extern crate libmodbus_rs;
extern crate failure;

use std::env;
use std::process;
use libmodbus_rs::{Modbus, ModbusClient, ModbusRTU};
use failure::Error;

pub fn main() -> Result<(), Error> {

  let args: Vec<String> = env::args().collect();
  
  if args.len() != 6 && args.len() != 7 {
    println!("Error! Missing arguments! Received: {:?} \nUsage example:", args);
    println!("./quick-modbus PORT_NAME    BAUD DEVICE_ADDRESS COMMAND        REGISTER [VALUE]");
    println!("./quick-modbus /dev/ttyUSB0 9600 1              write_register 1        1");
    println!("./quick-modbus /dev/ttyUSB0 9600 1              write_coil     1        1");
    println!("./quick-modbus /dev/ttyUSB0 9600 1              read_register  1         ");
    println!("./quick-modbus /dev/ttyUSB0 9600 1              read_coil      1         ");
    process::exit(1);
  }

  let port_name      = &args[1];
  let baud           =  args[2].parse::<i32>().unwrap();
  let device_address =  args[3].parse::<u8>().unwrap();
  let cmd            =  args[4].as_str();
  let register       =  args[5].parse::<u16>().unwrap();
  let mut value: u16 = 0;
  if args.len() == 7 { 
    value           =   args[6].parse::<u16>().unwrap();
  }

  let mut modbus = Modbus::new_rtu(port_name, baud, 'N', 8, 1).unwrap();
  modbus.set_slave(device_address)?;
  modbus.set_debug(true)?;

  modbus.connect()?;

  match cmd { 
    "read_register" => {
      // Function code: 0x03   Read single  Analog Output Holding Register
      println!("\nReading register: {:?}", register);
      let mut dest = vec![0u16; 1];
      assert!(modbus.read_registers(register, 1, &mut dest).is_ok());
      println!("Value read:{:?}", dest);
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
