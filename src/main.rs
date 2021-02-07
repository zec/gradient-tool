extern crate web_view;

use web_view::WVResult;

mod consts {
    include!(concat!(env!("OUT_DIR"), "/consts.rs"));
}

fn main() {
    let result = run_webview();

    if let Err(ref e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run_webview() -> WVResult<()> {
    use web_view::*;

    let mut wv = WebViewBuilder::new()
        .title("Color space gradient tool")
        .content(Content::Url(consts::UI_HTML))
        .size(600, 800)
        .user_data(())
        .invoke_handler(|_wv, arg| { println!("{}", arg); Ok(())})
        .build()?;

    wv.run()?;

    Ok(())
}
