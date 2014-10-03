#![feature(phase)]

extern crate mecab;
#[phase(plugin)] extern crate "link-config" as link_config;

link_config!("mecab", ["*-config"]) extern {}

fn main() {
    let mecab = mecab::Tagger::new2("");

    let input = "うらにわにはにわにわにはにわにわとりがいる";

    println!("input: {:s}", input);

    let node = mecab.parse_to_node(input);

    print!("output: ");

    for n in node.iter() {
        match n.get_status() {
            ::mecab::Unknown | ::mecab::Normal => print!("{:s} ", n.get_surface()),
            _ => {}
        }
    }

    print!("\n");
}
