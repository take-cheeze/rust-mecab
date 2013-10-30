extern mod std;
extern mod mecab;

use mecab::IMeCabNode;

fn main() {
    let mecab = mecab::new2("");

    let input = "うらにわにはにわにわにはにわにわとりがいる";

    std::io::println(format!("input: {:s}", input));

    let node = mecab.parse_to_node(input);

    std::io::print("output: ");

    for n in node.iter() {
        let status = unsafe { (*n).get_status() };

        if status == mecab::UNK_NODE || status == mecab::NOR_NODE {
            std::io::print(format!("{:s} ", unsafe { (*n).get_surface() } ));
        }
    }

    std::io::print("\n");
}
