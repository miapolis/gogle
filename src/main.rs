use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{io, thread, time};
use urlencoding::encode;

const GOOGLE_BASE_URL: &'static str = "https://google.com/search?q=";

fn main() {
    println!("GOGLE");
    let mut ctx = ClipboardContext::new().unwrap();
    let mut query = String::new();

    io::stdin()
        .read_line(&mut query)
        .expect("Failed to read line");

    let encoded = encode(&query);
    let url = format!("{}{}", GOOGLE_BASE_URL, encoded);
    ctx.set_contents(String::from(url)).unwrap();
    thread::sleep(time::Duration::from_millis(1));
}
