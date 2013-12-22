extern mod std;
extern mod extra;
extern mod mecab;

fn main() {
    let s = "これはテストです";
    let model = mecab::Model::new2("");
    let model = ~extra::arc::Arc::new(model);

    for unused in range(0,2) {
        let model = ~model.clone();

        do spawn {
            let model = model.get();
            let tagger = model.create_tagger();
            let lattice = model.create_lattice();

            lattice.set_sentence(s);

            if tagger.parse_lattice(&lattice) {
                println("result: ");
                println(format!("{:s}", lattice.to_str()));
            }
        }
    }

    let model = model.get();
    let tagger = model.create_tagger();
    let lattice = model.create_lattice();

    lattice.set_sentence(s);

    if tagger.parse_lattice(&lattice) {
        println("result: ");
        println(format!("{:s}", lattice.to_str()));
    }
}
