use clipboard::{ClipboardContext, ClipboardProvider};
use std::{thread, time};
use urlencoding::encode;

const GOOGLE_BASE_URL: &'static str = "https://google.com/search?q=";

fn main() {
    let mut ctx = ClipboardContext::new().unwrap();
    let mut query = ctx
        .get_contents()
        .expect("Failed to read clipboard contents as string.");
    query = query.trim().to_owned();

    let encoded = encode(&query);
    let url = format!("{}{}", GOOGLE_BASE_URL, encoded);
    ctx.set_contents(url).unwrap();

    thread::sleep(time::Duration::from_millis(20));
}
