#![feature(phase)]

extern crate mecab;
#[phase(plugin)] extern crate "link-config" as link_config;

link_config!("mecab") extern {}

fn main() {
    let mecab = ::mecab::Tagger::new2("");

    let input = "我々は、宇宙人だ";

    println!("input: {:s}", input);

    let node = mecab.parse_to_node(input);

    print!("output: ");

    for n in node.iter() {
        match n.get_status() {
            ::mecab::Normal => {
                print!("{:s}", n.get_feature().as_slice().split(',').nth(7).unwrap());
            }, _ => {}
        }
    }

    println!("");
}
