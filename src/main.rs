#![feature(socket_timeout)]
extern crate mio;
use mio::tcp::*;

use std::net::{TcpListener, TcpStream};
use std::net::UdpSocket;
use std::io::prelude::*;


use std::thread;
use std::env;

static ID: &'static str = "CUSTOMER_ID"; // A string representing the current executions unique ID
static HOST: &'static str = "SERVER_HOST"; // A <host>:<port> combination of a remote server
static PERIOD: &'static str = "REPORT_PERIOD"; // Number of seconds to buffer for before sending data to server
static REG_PORT: &'static str = "REGISTER_PORT"; // UNIX socket address for the register endpoint
static REP_PORT: &'static str = "REPORT_PORT"; // // UNIX socket address for the report endpoint

struct  Variables {
//    id: String,
//    host: String,
//    period: u16,
    reg_port: u16,
    rep_port: u16,
}

fn main() {
    let data =  match read_variable() {
        Ok(r) => r,
        Err(e) => panic!(e.to_string()),
    };

    let reg_port = data.reg_port;
    let rep_port = data.rep_port;


    // start udp server
    thread::spawn(move|| {
        start_udp(rep_port);
    });

    // start tcp server
    thread::spawn(move|| {
        start_tcp(reg_port);
    }).join();
}

fn read_variable() -> Result<Variables, String> {
//    let id = try!(env::var(ID).map_err(|e| e.to_string()));
//    let host = try!(env::var(HOST).map_err(|e| e.to_string()));
//    let str_period = try!(env::var(PERIOD).map_err(|e| e.to_string()));
    let str_reg_port =  try!(env::var(REG_PORT).map_err(|e| e.to_string()));
    let str_rep_port =  try!(env::var(REP_PORT).map_err(|e| e.to_string()));

//    let period = try!(str_period.parse::<u16>().map_err(|e| e.to_string()));
    let reg_port = try!(str_reg_port.parse::<u16>().map_err(|e| e.to_string()));
    let rep_port = try!(str_rep_port.parse::<u16>().map_err(|e| e.to_string()));

    let origin = Variables {
//        id: id,
//        host: host,
//        period: period,
        reg_port: reg_port,
        rep_port: rep_port
    };
    Ok(origin)
}

fn start_tcp(port: u16) {
    let tcp = TcpListener::bind(("127.0.0.1", port)).unwrap();
    println!("{}", port);
    for stream in tcp.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            },
            Err(e) => {
                println!("failed: {}", e);
            },
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf;
    loop {
        buf = [0; 512];
        let result = stream.read(&mut buf);

//        for x in buf.iter() {
//            println!("{}", x);
//        }

        match result {
            Ok(n) => {
                println!("received {} bytes", n);
                if n == 0 {
                    break;
                }
            },
            _ => {},
        }
    }
}

fn start_udp(port: u16) {
    let socket = UdpSocket::bind(("127.0.0.1", port)).unwrap();
    println!("{}", port);
}
