extern crate videos;
use std::fs::File;
use std::io::{Read, Write};
use videos::parser::parse;
use videos::output::produce_output;
use videos::algo::{Mode, algo};

fn main() {
    let files = vec!["kittens.in", "me_at_the_zoo.in", "trending_today.in", "videos_worth_spreading.in"];
    for file in files {
        println!("Processing file {}", file);
        process(file).unwrap();
    }
}

fn process(filename: &str) -> Result<(), String> {
    let in_filename = format!("resources/{}", filename);
    let out_filename = format!("output/{}.out", filename);
    File::open(in_filename)
        .map_err(|err| err.to_string())
        .and_then(|mut file| {
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .map_err(|err| err.to_string())
                .map(|_| contents)
        }).and_then(|contents| {
            parse(&*contents).ok_or(String::from("Unable to parse input"))
        }).and_then(|(cache_info, videos, endpoints, requests)| {
            let output = algo(Mode::Descent, cache_info, videos, endpoints, requests);
            let output_string = produce_output(output);

            File::create(out_filename)
                .map_err(|err| err.to_string())
                .and_then(|mut file| {
                    file.write_all(output_string.as_bytes())
                        .map_err(|err| err.to_string())
                        .map(|_| ())
                })
        })
}