use std::fmt::Display;
use std::io::prelude::*;
use std::net::TcpStream;

const DEBUG: bool = true;
const PORT: &str = "8001";
const SERVER: &str = "127.0.0.1";

// KISS Command codes
const DATAFRAME: u8 = 0x00;
const TXDELAY: u8 = 0x01;
const P: u8 = 0x02;
const SLOTTIME: u8 = 0x03;
const TXTAIL: u8 = 0x04;
const FULLDUPLEX: u8 = 0x05;
const SETHARDWARE: u8 = 0x06;
const RETURN: u8 = 0xFF;

// Special characters
const FEND: u8 = 0xC0;
const FESC: u8 = 0xDB;
const TFEND: u8 = 0xDC;
const TFESC: u8 = 0xDD;


struct NODE {
    beacon: bool,
    callsign: String,
    sysop: String,
}

impl NODE {
    fn new() -> Self {
        Self {
            beacon: false,
            callsign: String::from("GB7ZAH"),
            sysop: String::from("M0ZAH"),
        }
    }
}

fn debug<T: Display>(msg: T){
    if DEBUG {
        println!("{}", msg);
    }
}

fn main() -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    let mynode = NODE::new();
    debug("DEBUG Enabled");
    println!("Node Details\nCallsign: {}\nSysop: {}", mynode.callsign, mynode.sysop);

    let mut stream = TcpStream::connect("127.0.0.1:8001")?;
    stream.read(&mut buffer[..])?;
    let data = std::str::from_utf8(&buffer).unwrap();
    println!("{}", data);
    Ok(())
}
