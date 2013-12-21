extern mod std;
extern mod mecab;

use mecab::INode;

fn main() {
    let mecab = mecab::Tagger::new2("");

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
