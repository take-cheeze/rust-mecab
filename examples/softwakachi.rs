#![feature(phase)]

extern crate mecab;
#[phase(plugin)] extern crate "link-config" as link_config;

link_config!("mecab") extern {}

fn main() {
    let mecab = ::mecab::Tagger::new2("-m -a");
    let node = mecab.parse_to_node("東京特許許可局");
    for node in node.iter() {
        let prob = node.get_prob();
        match node.get_status() {
            ::mecab::Normal =>
                if node.is_best() || prob >= 0.05 {
                    let surface = node.get_surface();
                    let feature = node.get_feature();
                    println!("{}\t{}\t{}", surface, feature, prob);
                },
            _ => {}
        }
    }
}
