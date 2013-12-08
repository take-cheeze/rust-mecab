extern mod std;
extern mod mecab;

use mecab::IMeCabNode;

//use std::rt::io::print;
//use std::rt::io::println;
/*
fn get_tagger() -> &mecab::MeCab {
    let model = mecab::MeCabModel::new2("");
    model.create_tagger()
}
*/

fn main() {
    let mut model = mecab::MeCabModel::new2("");
    let mecab = model.create_tagger();
    //let mecab = mecab::MeCab::new2("");
    //let mecab = get_tagger();

    let input = "うらにわにはにわにわにはにわにわとりがいる";

    println(format!("input: {:s}", input));

    let node = mecab.parse_to_node(input);

    print("output: ");

    for n in node.iter() {
        let status = unsafe { (*n).get_status() };

        if status == mecab::UNK_NODE || status == mecab::NOR_NODE {
            print(format!("{:s} ", unsafe { (*n).get_surface() } ));
        }
    }

    print("\n");
}
