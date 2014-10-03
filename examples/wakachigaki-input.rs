#![feature(phase)]

extern crate mecab;
#[phase(plugin)] extern crate "link-config" as link_config;

link_config!("mecab", ["*-config"]) extern {}

fn main() {
    let model = mecab::Model::new2("");
    let mecab = model.create_tagger();
    //let mecab = mecab::Tagger::new2("");
    //let mecab = get_tagger();

//    let input = "うらにわにはにわにわにはにわにわとりがいる";
//


    let mut stdin = std::io::stdio::stdin();



    for line in stdin.lines() {
        let line = line.unwrap();
        print!("\n");
        println!("input: {:s}", line);

        let node = mecab.parse_to_node(line.as_slice());
        print!("output: ");
        for n in node.iter() {
            match n.get_status() {
                ::mecab::Normal | ::mecab::Unknown =>
                    print!("{:s} ", n.get_surface()),
                _ => {}
            }
        }
        print!("\n");
    }

}
