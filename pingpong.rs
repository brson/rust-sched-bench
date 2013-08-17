// This is a simple bench that creates M pairs of of tasks. These
// tasks ping-pong back and forth over a pair of streams. This is a
// cannonical message-passing benchmark as it heavily strains message
// passing and almost nothing else.

use std::cell::Cell;

fn ping_pong_bench(n: uint, m: uint) {

    // Create pairs of tasks that pingpong back and forth.
    fn run_pair(n: uint) {
        // Create a stream A->B
        let (pa,ca) = stream::<()>();
        // Create a stream B->A
        let (pb,cb) = stream::<()>();

        let pa = Cell::new(pa);
        let ca = Cell::new(ca);
        let pb = Cell::new(pb);
        let cb = Cell::new(cb);

        do spawn {
            use std::task;
            do task::unkillable {
                let chan = ca.take();
                let port = pb.take();
                do n.times {
                    chan.send(());
                    port.recv();
                }
            }
        }

        do spawn {
            use std::task;
            do task::unkillable {
                let chan = cb.take();
                let port = pa.take();
                do n.times {
                    port.recv();
                    chan.send(());
                }
            }
        }
    }

    do m.times {
        run_pair(n)
    }

}

fn main() {
    ping_pong_bench(100000, 16);
}
