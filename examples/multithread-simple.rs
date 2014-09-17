#![feature(phase)]

extern crate mecab;
#[phase(plugin)] extern crate "link-config" as link_config;

link_config!("mecab") extern {}

fn main() {
    let s = "これはテストです";

    for _ in range(0,2u) {
        spawn(proc() {
            let model = ::mecab::Model::new2("");
            let tagger = model.create_tagger();
            let mut lattice = model.create_lattice();

            lattice.set_sentence(s);

            if tagger.parse_lattice(&lattice) {
                println!("result: ");
                println!("{:s}", lattice.to_string());
            }
        })
    }

    let model = ::mecab::Model::new2("");
    let tagger = model.create_tagger();
    let mut lattice = model.create_lattice();

    lattice.set_sentence(s);

    if tagger.parse_lattice(&lattice) {
        println!("result: ");
        println!("{:s}", lattice.to_string());
    }
}
