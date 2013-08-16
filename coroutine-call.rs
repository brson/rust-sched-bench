extern mod extra;

use std::comm::{ChanOne, oneshot};
use extra::time::precise_time_ns;

enum LayoutMsg {
    GetBoundingClientRect(ChanOne<int>),
    Exit
}

static NUM_CALLS: uint = 50000;

fn main() {
    run_test();
}

fn run_test() {
    let (layout_port, layout_chan) = stream();

    start_layout_task(layout_port);

    do time("coroutine call") {
        let (response_port, response_chan) = oneshot();
        layout_chan.send(GetBoundingClientRect(response_chan));
        response_port.recv();
    }
    layout_chan.send(Exit);

    do time("function call") {
        get_bounding_client_rect();
    }
}

fn start_layout_task(port: Port<LayoutMsg>) {
    do spawn {
        loop {
            match port.recv() {
                GetBoundingClientRect(response_chan) => {
                    let res = get_bounding_client_rect();
                    response_chan.send(res)
                }
                Exit => break
            }
        }
    }
}

#[inline(never)]
fn get_bounding_client_rect() -> int {
    return 0;
}

#[inline(always)]
fn time(desc: &str, f: &fn()) {
    let start = precise_time_ns();
    do NUM_CALLS.times {
        f();
    }
    let end = precise_time_ns();
    let total = end - start;
    println(fmt!("%s num calls: %u", desc, NUM_CALLS));
    println(fmt!("%s total ns: %u", desc, total as uint));
    println(fmt!("%s ns per call: %f", desc, (total as float / (NUM_CALLS as float))));
}
