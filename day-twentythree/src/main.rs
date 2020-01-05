use std::collections::VecDeque;
use std::sync::mpsc;
use std::thread;
use std::time;

fn main() {
    const NUM_COMPS: usize = 50;

    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let line = std::fs::read_to_string(&path).expect("file not found");

    let tape: Vec<i64> = intcode::to_tape(&line);

    // Make a transmitter/receiver pair for each computer.  We put the
    // transmitters into a Vec since we're going to need to clone each one for
    // each computer we make (so that every computer can transmit to every other
    // computer).  The receivers, however, can go into a VecDeque so that we can
    // easily pop them off and move them into the corresponding computer.
    let mut txs: Vec<mpsc::Sender<(i64, i64)>> = Vec::with_capacity(NUM_COMPS+2);
    let mut rxs: VecDeque<mpsc::Receiver<(i64, i64)>> = VecDeque::with_capacity(NUM_COMPS);
    for _ in 0..NUM_COMPS {
        let (tx, rx) = mpsc::channel::<(i64, i64)>();
        txs.push(tx);
        rxs.push_back(rx);
    }

    // Add two more channels that the threads can use to pass messages to the
    // NAT.  The first is to send packets, and the second is to communicate idleness.
    let (nat_tx, nat_rx) = mpsc::channel::<(i64, i64)>();
    txs.push(nat_tx);

    let (state_tx, state_rx) = mpsc::channel::<(i64, bool)>();

    for i in 0..NUM_COMPS {
        // Clone the tape now so that we don't accidentally move the master
        // into one of the threads.
        let tape_clone = tape.clone();

        // Remove the corresponding receiver.
        let temp_rx = rxs.pop_front().unwrap();

        // Clone all the transmitters into a new list for the computer.
        let mut temp_txs: Vec<mpsc::Sender<(i64, i64)>> = Vec::with_capacity(NUM_COMPS+1);
        for tx in &txs {
            temp_txs.push(mpsc::Sender::clone(&tx));
        }

        // Clone the state transmitter
        let temp_state_tx = mpsc::Sender::clone(&state_tx);

        // Spawn a new thread to hold one of the computers.
        thread::spawn(move || {
            let addr = i as i64;
            let rx = temp_rx;
            let txs = temp_txs;
            let state_tx = temp_state_tx;
            let mut comp = intcode::IntcodeComp::new(tape_clone);
            comp.push_input(addr);

            let mut is_idle = false;
            loop {
                // Let the computer do it's thing.
                comp.start();

                // Once it's done, push out published messages first.
                let mut new_is_idle = true;
                while let Some(dest) = comp.pop_output() {
                    new_is_idle = false;
                    let x = comp.pop_output().unwrap();
                    let y = comp.pop_output().unwrap();

                    if dest == 255 {
                        txs[50].send((x, y)).unwrap();
                    } else if dest < 50 {
                        txs[dest as usize].send((x, y)).unwrap();
                    } else {
                        panic!("Unexpected address {}", dest);
                    }
                }

                // Then try to get a pending message.  If there are none, then just
                // provide -1 as a value.
                let maybe_pair = rx.try_recv();
                match maybe_pair {
                    Ok((x, y)) => {
                        new_is_idle = false;
                        comp.push_input(x);
                        comp.push_input(y);
                    },
                    Err(_) => comp.push_input(-1),
                }

                // Inform the NAT if this computer's idleness changed.
                if is_idle != new_is_idle {
                    is_idle = new_is_idle;
                    state_tx.send((addr, is_idle)).unwrap();
                }
            }
        });
    }

    // Bits that are 'on' represent idle computers.
    let mut state_field = 0_u64;
    let state_mask = 2_u64.pow(50) - 1;
    let mut idleness_count = 0;
    let mut packet = (0, 0);
    let mut prev_packet = (0, 0);
    loop {
        // Update the state field for any computers that have changed idleness.
        thread::sleep(time::Duration::from_millis(50));
        while let Ok((addr, is_idle)) = state_rx.try_recv() {
            let bit: u64 = 1 << addr;
            if is_idle {
                state_field |= bit;
            } else {
                state_field &= !bit;
            }
        }

        // Keep receiving packets till we get the last one that came in.
        thread::sleep(time::Duration::from_millis(50));
        while let Ok(p) = nat_rx.try_recv() {
            println!("Got packet {:?}", p);
            packet = p;
        }

        // Make sure all comps have been idle for a full second, to squash
        // raceyness.
        if state_field == state_mask {
            idleness_count += 1;
        } else {
            idleness_count = 0;
        }
        if idleness_count == 10 {
            println!("All computers idle, sending packet {:?}", packet);
            txs[0].send(packet).unwrap();
            idleness_count = 0;
            if packet.1 == prev_packet.1 { break; }
            prev_packet = packet;
        }
    }
}
