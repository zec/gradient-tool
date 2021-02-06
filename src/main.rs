extern crate web_view;

use web_view::{WebViewBuilder,WVResult};
use web_view::Content::*;

fn main() {
    println!("Hello, world!");

    match run_webview() {
      Ok(_)  => println!("ran successfully"),
      Err(e) => println!("Error: {}", e)
    }
}

fn run_webview() -> WVResult<()> {
    let mut wv = WebViewBuilder::new()
        .title("Hi there!")
        .content(Html("<html><head><title>.</title><body><p>Hello, world</p></body></html>"))
        .size(600, 800)
        .user_data(())
        .invoke_handler(|_wv, _arg| Ok(()))
        .build()?;

    wv.run()?;

    Ok(())
}
