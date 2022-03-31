extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;
use regex::Regex;

#[proc_macro_derive(HelloWorld)]
pub fn hello_world(input: TokenStream) ->TokenStream {
    let s = input.to_string();
    let name = parse_struct_name(&s);
    let output = format!(r#"
impl THelloWorld for {0} {{
    fn hello() {{ println!(" {0} says hello "); }}
}}"#, name);
    TokenStream::from_str(&output).unwrap()
}

fn parse_struct_name(s: &str) -> String {
    let r = Regex::new(r"(?:struct\s+)([\w\d_]+)").unwrap();
    let caps = r.captures(s).unwrap();
    caps[1].to_string()
}

#[test]
fn test_parse_struct_name() {
    let input = "struct Foo(i32);";
    let name = parse_struct_name(input);
    assert_eq!(&name, "Foo");
}
