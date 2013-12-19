extern mod std;
extern mod mecab;

use mecab::INode;

fn main() {
    let mut model = mecab::Model::new2("");
    let mecab = model.create_tagger();
    //let mecab = mecab::Tagger::new2("");
    //let mecab = get_tagger();

//    let input = "うらにわにはにわにわにはにわにわとりがいる";
//


//    let mut stdin = std::io::buffered::BufferedReader::new(std::io::stdin());
    let mut stdin = std::io::buffered::BufferedReader::new(std::io::stdin());



    for line in stdin.lines() {
        print("\n");
        println(format!("input: {:s}", line));

        let node = mecab.parse_to_node(line);
        print("output: ");
        for n in node.iter() {
            let status = unsafe { (*n).get_status() };

            if status == mecab::UNK_NODE || status == mecab::NOR_NODE {
                print(format!("{:s} ", unsafe { (*n).get_surface() } ));
            }
        }
        print("\n");
    }

}
