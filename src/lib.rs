use wasm_bindgen::prelude::*;
use std::borrow::Cow;
use regex::Regex;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    // Debugging
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
pub fn popup(text: &str) {
    alert(&text);
}

/// Fast replace by Christopher
pub fn replacen<'t>(
    reg: &Regex,
    text: &'t str,
    limit: usize,
    rep1: &str,
    rep2: &str,
) -> Cow<'t, str> {
        let mut it = reg.find_iter(text).enumerate().peekable();
        if it.peek().is_none() {
            return Cow::Borrowed(text);
        }
        let mut new = String::with_capacity(text.len());
        let mut last_match = 0;
        for (i, m) in it {
            if limit > 0 && i >= limit {
                break;
            }
            new.push_str(&text[last_match..m.start()]);
            new.push_str(&rep1);
            new.push_str(&text[m.start()..m.end()]);
            new.push_str(&rep2);
            last_match = m.end();
        }
        new.push_str(&text[last_match..]);
        return Cow::Owned(new);
}

#[wasm_bindgen]
pub fn highlight(source: &str) -> String {
    
    let re = Regex::new(r"([A-Z])\w+").unwrap();
    String::from(replacen(&re, source, 0, "<span class=\"comment\">", "</span>"))
}