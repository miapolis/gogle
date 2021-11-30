use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::{thread, time};
use urlencoding::encode;

const GOOGLE_BASE_URL: &'static str = "https://google.com/search?q=";

fn main() {
    println!("GOGLE");
    let ctx = ClipboardContext::new().unwrap();
    let query = ctx.get_contents().unwrap();

    let encoded = encode(&query);
    let url = format!("{}{}", GOOGLE_BASE_URL, encoded);
    ctx.set_contents(url).unwrap();
    thread::sleep(time::Duration::from_millis(20));
}
