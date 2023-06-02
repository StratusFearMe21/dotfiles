use std::io::{stdout, BufWriter, Write};

use calloop::EventLoop;
use rustbar::{SharedData, SharedDataTransparent};

fn main() {
    let mut event_loop: EventLoop<_> = EventLoop::try_new().unwrap();
    let mut stdout = BufWriter::new(stdout().lock());
    stdout.write_all(b"{ \"version\": 1 }\n[\n[]").unwrap();
    let mut shared_data =
        SharedDataTransparent(SharedData::new(event_loop.get_signal(), move |string| {
            stdout.write_all(b",[{\"full_text\":\"").unwrap();
            stdout.write_all(&string).unwrap();
            stdout.write_all(b"\"}]").unwrap();
            stdout.flush().unwrap();
        }));
    rustbar::insert_into_loop(&mut event_loop, &mut shared_data);

    event_loop.run(None, &mut shared_data, |_| {}).unwrap();
}
