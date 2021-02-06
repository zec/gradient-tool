extern crate web_view;

use web_view::WebViewBuilder;
use web_view::Content::*;

fn main() {
    println!("Hello, world!");

    let wv = WebViewBuilder::new()
        .title("Hi there!")
        .content(Html("<html><head><title>.</title><body><p>Hello, world</p></body></html"))
        .size(500, 900)
        .user_data(())
        .invoke_handler(|_wv, _arg| Ok(()))
        .build().unwrap();

    wv.run().unwrap();
}
