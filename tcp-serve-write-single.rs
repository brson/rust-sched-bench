use core::libc::*;
use core::rt::sched::*;
use core::rt::uv::uvio::*;
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
static NUM_WRITES: uint = 64 * 1024;

#[start]
fn start(_argc: int, _argv: **u8, _crate_map: *u8) -> int {
    let loop_ = ~UvEventLoop::new();
    let mut sched = ~Scheduler::new(loop_);
    let main_task = ~Task::new(&mut sched.stack_pool, serve);
    sched.task_queue.push_back(main_task);
    sched.run();
    return 0;
}

fn serve() {
    let addr = Ipv4(127, 0, 0, 1, 9001);
    let mut listener = TcpListener::bind(addr).unwrap();
    let mut stream = listener.accept().unwrap();
    let buf = vec::from_fn(BUF_SIZE, |i| (i % 64 + 48) as u8);
    for NUM_WRITES.times {
        stream.write(buf);
    }
    rtdebug!("wrote %u bytes", buf.len() * NUM_WRITES);
}
