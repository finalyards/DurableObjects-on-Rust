mod outer;
mod weather;

use worker::{ console_debug, event };

#[event(start)]
fn start() {
    console_debug!("DO start")
}
