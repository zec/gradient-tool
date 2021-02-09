/*
   Copyright 2021 Zachary Catlin

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.

*/

extern crate web_view;
extern crate webbrowser;

use web_view::WVResult;

fn main() {
    let result = run_webview();

    if let Err(ref e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

const UI_HTML: &'static str = include_str!("../ui.html");

fn run_webview() -> WVResult<()> {
    use web_view::*;

    let mut wv = WebViewBuilder::new()
        .title("Color space gradient tool")
        .content(Content::Url("data:text/plain,%20"))
        .size(600, 800)
        .min_size(600, 50)
        .user_data(())
        .invoke_handler(|_wv, arg| { drop(webbrowser::open(arg)); Ok(())})
        .build()?;

    wv.set_html(UI_HTML)?;
    wv.run()?;

    Ok(())
}
