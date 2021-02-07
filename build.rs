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

extern crate urlencoding;

use std::{env,mem};
use std::{fs::File,io::BufWriter,io::Write};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let f_name = format!("{}/consts.rs", out_dir);

    println!("cargo:rerun-if-changed={}/build.rs", src_dir);
    println!("cargo:rerun-if-changed={}", f_name);

    let mut f = BufWriter::new(File::create(f_name).unwrap());

    read_and_encode_file(
        &format!("{}/ui.html", src_dir),
        "UI_HTML", "text/html;charset=UTF-8",
        &mut f
    );

    f.flush().unwrap();
    mem::drop(f);
}

fn read_in_file(f_name: &str) -> String {
    println!("cargo:rerun-if-changed={}", f_name);
    let v: Vec<u8> = std::fs::read(f_name).unwrap();
    String::from_utf8(v).unwrap()
}

fn read_and_encode_file<W: Write>(
    src_fname: &str,
    var_name: &str, mime_type: &str,
    out_file: &mut W) {

    let s = read_in_file(src_fname);

    write!(
        out_file,
        "pub(crate) const {var}: &'static str = &r#\"data:{mt},{content}\"#;\n",
        var = var_name, mt = mime_type, content = urlencoding::encode(&s)
    ).unwrap();
}
