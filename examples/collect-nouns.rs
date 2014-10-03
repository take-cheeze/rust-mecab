#![feature(phase)]

extern crate mecab;
#[phase(plugin)] extern crate "link-config" as link_config;

use std::comm;

link_config!("mecab", ["*-config"]) extern {}

fn collect_nouns(lattice: &::mecab::Lattice) -> Vec<String> {
    let mut v = Vec::new();

    let node = lattice.get_bos_node();
    for n in node.iter() {
        match n.get_status() {
            ::mecab::Normal | ::mecab::Unknown => {
                let feature = n.get_feature();
                if feature.as_slice().split(',').nth(0).unwrap() == "名詞" {
                    v.push(n.get_surface().to_string());
                }
            }, _ => {}
        }
    }
    return v;
}

fn main() {
    let sentences = [
        "これはテストです",
        "太郎は次郎が持っている本を花子に渡した",
        "昨日の夕食はカレーでした",
    ];

    let (c, p) = comm::channel();

    for &sentence in sentences.iter() {
        let c = c.clone();
        spawn(proc() {
            let model = ::mecab::Model::new2("");
            let tagger = model.create_tagger();
            let mut lattice = model.create_lattice();

            lattice.set_sentence(sentence);

            if tagger.parse_lattice(&lattice) {
                c.send(collect_nouns(&lattice));
            }
        })
    }

    for _ in range(0, sentences.len()) {
        for noun in p.recv().iter() {
            println!("noun: {}", *noun);
        }
    }
}
