#[test]
fn test_mecab_version() {
    let vers = ::version();
    assert!(!vers.is_empty());
}

#[test]
fn test_mecab_dictionary_info() {
    let m = ::Tagger::new2("");

    for d in m.get_dictionary_info().iter() {
        println!("filename: {:s}", d.get_filename());
        println!("charset:  {:s}", d.get_charset());
        println!("size:     {:?}", d.get_size());
        println!("type:     {:?}", d.get_type());
        println!("lsize:    {:?}", d.get_lsize());
        println!("rsize:    {:?}", d.get_rsize());
        println!("version:  {:?}", d.get_version());
    }
}

#[test]
fn test_mecab_parse() {
    let mecab = ::Tagger::new2("");
    let s = mecab.parse("この文はテストです");
    println!("{:s}", s);
}

#[test]
fn test_mecab_parse_to_node() {
    let mecab = ::Tagger::new2("");

    for n in mecab.parse_to_node("この文はテストです").iter() {
        match n.get_status() {
            ::Normal | ::Unknown =>
                println!("surface: {:s}, feature: {:s}", n.get_surface(), n.get_feature()),
            _ => {}
        }
    }
}
