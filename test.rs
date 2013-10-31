
#[test]
fn test_mecab_version() {
    let vers = version();
    assert!(!vers.is_empty());
}

#[test]
fn test_mecab_dictionary_info() {
    let mecab = new2("");
    let dict  = mecab.get_dictionary_info();

    for d in cont.iter() {
        io::println(format!("filename: {:s}", d.get_filename()));
        io::println(format!("charset:  {:s}", d.get_charset()));
        io::println(format!("size:     {:?}", d.get_size()));
        io::println(format!("type:     {:?}", d.get_type()));
        io::println(format!("lsize:    {:?}", d.get_lsize()));
        io::println(format!("rsize:    {:?}", d.get_rsize()));
        io::println(format!("version:  {:?}", d.get_version()));
    }
}

#[test]
fn test_mecab_parse() {
    let mecab = new2("");
    let s = mecab.parse("この文はテストです");
    io::println(format!("{:s}", s));
}

#[test]
fn test_mecab_parse_to_node() {
    let mecab = new2("");
    let node = mecab.parse_to_node("この文はテストです");

    for n in node.iter() {
        let status = n.get_status();
        if status == NOR_NODE || status == UNK_NODE {
            io::println(format!("surface: {:s}", n.get_surface()));
        }
    }
}
