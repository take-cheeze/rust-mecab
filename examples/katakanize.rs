extern mod mecab;

use mecab::INode;

fn main() {
    let mecab = mecab::Tagger::new2("");

    let input = "我々は、宇宙人だ";

    println(format!("input: {:s}", input));

    let node = mecab.parse_to_node(input);

    print("output: ");

    for n in node.iter() {
        let status = unsafe { (*n).get_status() };

        if status == mecab::NOR_NODE {
            let mut i = 0;
            let feature = unsafe { (*n).get_feature() };
            for s in feature.split(',') {
                if i == 7 { print(format!("{:s}", s)); }
                i += 1;
            }
        }
    }

    println("");
}
