use clap::{App, Arg};
use std::io;
use std::io::{Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::str;
use std::thread;

fn handle_stream(local_stream: &mut TcpStream) -> Option<TcpStream> {
    let mut buffer = [0u8; 256];
    if local_stream.read(&mut buffer[..]).is_err() {
        return None;
    }
    if buffer[0] != 5 {
        return None;
    }
    if local_stream.write(&mut [5u8, 0u8]).is_err() {
        return None;
    }

    let ret = local_stream.read(&mut buffer[..]);
    if ret.is_err() {
        return None;
    }
    let n = ret.unwrap();
    if buffer[0] != 5u8 {
        return None;
    }
    if buffer[1] != 1u8 {
        return None;
    }
    if buffer[2] != 0u8 {
        return None;
    }

    let host: String;
    let port: u16;
    let addr: SocketAddr;

    match buffer[3] {
        1u8 => {
            host = format!("{}.{}.{}.{}", buffer[4], buffer[5], buffer[6], buffer[7]);
            port = ((buffer[8] as u16) << 8) + buffer[9] as u16;
            let url = format!("{}:{}", host, port);
            addr = url.parse().unwrap();
        }
        3u8 => {
            let ndomain = buffer[4] as usize;
            host = str::from_utf8(&buffer[5..ndomain + 5]).unwrap().to_string();
            port = ((buffer[5 + ndomain] as u16) << 8) + buffer[6 + ndomain] as u16;

            let tmp = format!("{}:{}", host, port);
            if tmp.to_socket_addrs().is_ok() {
                let tmp = tmp.to_socket_addrs();
                if tmp.is_ok() {
                    addr = tmp.unwrap().as_slice()[0];
                } else {
                    buffer[0] = 5u8;
                    buffer[1] = 3u8;
                    local_stream.write(&buffer[..n]).unwrap_or_default();
                    return None;
                }
            } else {
                buffer[0] = 5u8;
                buffer[1] = 3u8;
                local_stream.write(&buffer[..n]).unwrap_or_default();
                return None;
            }
        }
        _ => {
            return None;
        }
    };
    println!("connect {}:{}", host, port);

    let ret = TcpStream::connect(&addr);

    if ret.is_err() {
        buffer[0] = 5u8;
        buffer[1] = 3u8;
        local_stream.write(&buffer[..n]).unwrap_or_default();
        return None;
    }
    buffer[0] = 5u8;
    buffer[1] = 0u8;
    if local_stream.write(&buffer[..n]).is_err() {
        return None;
    }

    return Some(ret.unwrap());
}

fn forward_stream(dst: TcpStream, src: TcpStream) {
    let mut l2 = dst;
    let mut r2 = src;
    let mut buffer2 = [0u8; 1024];
    loop {
        let mut len: usize;
        let ret = r2.read(&mut buffer2[..]);
        len = 0;
        if ret.is_ok() {
            len = ret.unwrap();
        }
        if len == 0 {
            l2.shutdown(Shutdown::Both).unwrap_or_default();
            r2.shutdown(Shutdown::Both).unwrap_or_default();
            break;
        }
        let ret = l2.write(&mut buffer2[..len]);
        len = 0;
        if ret.is_ok() {
            len = ret.unwrap();
        }
        if len == 0 {
            l2.shutdown(Shutdown::Both).unwrap_or_default();
            r2.shutdown(Shutdown::Both).unwrap_or_default();
            break;
        }
    }
}

fn listen_and_serve(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr)?;

    loop {
        let (mut local, _addr) = listener.accept()?;
        thread::spawn(move || {
            let ret = handle_stream(&mut local);
            if !ret.is_none() {
                let remote = ret.unwrap();
                let r1 = remote;
                let l1 = local;
                let r2 = r1.try_clone().unwrap();
                let l2 = l1.try_clone().unwrap();

                thread::spawn(move || {
                    forward_stream(r1, l1);
                });
                thread::spawn(move || {
                    forward_stream(l2, r2);
                });
            }
        });
    }
}

fn main() {
    let matches = App::new("rust-socks5")
        .version("0.1.0")
        .author("caiyesd <caiyesd@gmail.com>")
        .about("A rust implementation of socks5 proxy")
        .arg(
            Arg::with_name("local-address")
                .short("l")
                .long("local-address")
                .value_name("ADDR")
                .help("local listen address")
                .default_value("127.0.0.1:1080")
                .takes_value(true),
        )
        .get_matches();

    let addr = matches.value_of("local-address").unwrap();
    println!("starting socks5 proxy on {}", addr);
    match listen_and_serve(addr) {
        Err(err) => {
            println!("cannot listen on {}, error: {}", addr, err);
        }
        _ => {}
    }
}