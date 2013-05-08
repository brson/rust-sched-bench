use core::rt::io::*;
use core::rt::io::net::tcp::TcpListener;
use core::rt::io::net::ip::Ipv4;

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

static BUF_SIZE: uint = 64 * 1024;
static NUM_BYTES: uint = 1024 * BUF_SIZE;

#[start]
fn start(argc: int, argv: **u8, crate_map: *u8) -> int {
    do core::rt::start(argc, argv, crate_map) {
        let addr = Ipv4(127, 0, 0, 1, 9001);
        let mut listener = TcpListener::bind(addr).unwrap();
        let mut stream = listener.accept().unwrap();
        let mut buf = [0,.. 64 * 1024];
        let mut bytes = 0;
        loop {
            let nread = stream.read(buf);
            bytes += nread.get();
            if bytes >= NUM_BYTES { break; }
        }
        rtdebug!("read %u bytes", bytes);
    }
}
