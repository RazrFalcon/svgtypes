#![allow(dead_code)]

#[macro_use]
extern crate bencher;
extern crate svgtypes;

use std::fs;
use std::env;
use std::io::Read;

use bencher::Bencher;

fn load_string(path: &str) -> String {
    let path = env::current_dir().unwrap().join(path);
    let mut file = fs::File::open(&path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}

fn path_large(bencher: &mut Bencher) {
    let text = load_string("path-large.txt");
    bencher.iter(|| {
        for t in svgtypes::PathParser::from(text.as_str()) {
            let _ = t.unwrap();
        }
    })
}

benchmark_group!(paths, path_large);
benchmark_main!(paths);
