extern mod std;

use core::rt::io::*;
use core::rt::io::net::tcp::TcpListener;
use core::rt::io::net::ip::Ipv4;
use core::cell::Cell;
use std::arc;
use std::arc::ARC;

macro_rules! rtdebug (
    ($( $arg:expr),+) => ( {
        dumb_println(fmt!( $($arg),+ ));

        fn dumb_println(s: &str) {
            use core::io::WriterUtil;
            let dbg = ::libc::STDERR_FILENO as ::io::fd_t;
            dbg.write_str(s);
            dbg.write_str("\n");
        }

    } )
)

static CONNECTIONS: uint = 100;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    do core::rt::start(argc, argv, crate_map) {
        let dickens = ARC(io::read_whole_file_str(&Path("./bleak-house.txt")).unwrap());

        let addr = Ipv4(127, 0, 0, 1, 9001);
        let mut listener = TcpListener::bind(addr).unwrap();
        for CONNECTIONS.times {
            let stream = Cell(listener.accept().unwrap());
            let subdickens = dickens.clone();
            do spawn {
                let mut stream = stream.take();
                let dickens_str = arc::get(&subdickens);
                let dickens_bytes = str::as_bytes_slice(*dickens_str);
                stream.write(dickens_bytes);
            }
        }
    }
}
