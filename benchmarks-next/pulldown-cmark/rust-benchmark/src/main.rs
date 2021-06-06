use pulldown_cmark::{html, Options, Parser};
use sightglass_api as bench;

fn main() {
    let markdown_input = std::fs::read_to_string("./default.input.md").unwrap();

    let mut options = Options::empty();

    bench::start();

    let mut html_output = String::new();
    for _ in 0..500 {
        let parser = Parser::new_ext(&markdown_input, options);
        html_output.clear();
        html::push_html(&mut html_output, parser);
    }

    bench::end();

    eprintln!("{}", html_output);
}
