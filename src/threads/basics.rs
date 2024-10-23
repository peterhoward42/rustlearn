#[cfg(test)]

mod tests {

    /* Test Function Requirements

    Main thread spawns two others.
    One sends integers back through a channel at randomised short intervals, until it has sent 10, then exits.
    The other does the same with bool values.
    The main thread loops-forever trying to read from both these channels simultaneously,
    and harvests the values sent.
    The main thread loop detects when each of the threads terminates (Disconnected error),
    and waits for both threads to do so before ending itself.
    */

    use std::time::{SystemTime, UNIX_EPOCH};
    use std::{sync::mpsc, thread, time::Duration};

    #[test]
    fn basics_of_threads_and_channels() {
        let (tx_int, rc_int) = mpsc::channel();
        let (tx_bool, rc_bool) = mpsc::channel();

        thread::spawn(move || {
            send_integers(tx_int);
        });
        thread::spawn(move || {
            send_bools(tx_bool);
        });

        let mut captured_ints = Vec::new();
        let mut captured_bools = Vec::new();

        let mut finished_ints = false;
        let mut finished_bools = false;

        loop {
            match rc_int.try_recv() {
                // If we get one on this iteration harvest it.
                Ok(n) => captured_ints.push(n),
                // Error signifies either that the sending thread has finished, or
                // that nothing is available right now
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => {}
                    mpsc::TryRecvError::Disconnected => finished_ints = true,
                },
            }
            match rc_bool.try_recv() {
                Ok(b) => captured_bools.push(b),
                Err(e) => match e {
                    mpsc::TryRecvError::Empty => {}
                    mpsc::TryRecvError::Disconnected => finished_bools = true, // The sending thread finished.
                },
            }

            // Exit the loop once all ints and all bools received.
            if finished_ints && finished_bools {
                break;
            }
        }

        assert_eq!(captured_ints.len(), 10);
        assert_eq!(captured_bools.len(), 6);
    }

    // See also
    // - using clone() to have multiple transmitters.
    // - using join() to:
    //  - block this thread until all of N other threads have completed
    //  - receive the thread entrypoint function's return value
    //  - iterating over a receiver as a way to detect the sending channel's closure.
    // Synchronous threads

    // Sends a few integers down the given channel, with a short and variable delay
    // between each.
    fn send_integers(ch: mpsc::Sender<i32>) {
        for n in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
            thread::sleep(varying_short_delay());
            match ch.send(n) {
                Ok(()) => {}
                Err(e) => panic!("unexpected send error: {e}"),
            }
        }
    }

    // Sends a few bools down the given channel, with a short and variable delay
    // between each.
    fn send_bools(ch: mpsc::Sender<bool>) {
        for b in [true, false, false, true, false, true] {
            thread::sleep(varying_short_delay());
            match ch.send(b) {
                Ok(()) => {}
                Err(e) => panic!("unexpected send error: {e}"),
            }
        }
    }

    // Provides a Duration of between 0 and 499 microseconds.
    // The value you get creates the illusion of randomness, but it actually
    // varies (deterministicly) according to the time now() relative to the Epoch (on the host).
    //
    // todo - puzzled why the language does not provide a random number generator that
    // can do this - but I do understand about the emphasis on DDOS resilience.
    //
    // See also random number generator test in ./thirdpartycrate - this uses the "fastrand" crates.io package.
    fn varying_short_delay() -> Duration {
        // This deliberately uses unwrap() in full knowledge of its capacity to
        // panic - that is fine for a test.
        let delay: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros()
            % 500;

        // Note here, that you can call the try_into() (a generic type conversion method) on a plain u128 without
        // specifying the destination type!
        // Rust is able to infer the destination type - in this case from the delay() function's signature for
        // the return type.
        Duration::from_micros(delay.try_into().unwrap())
    }
}
