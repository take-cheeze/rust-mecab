/*!
MeCab bindings for Rust

Copyright (C) 2012 Tycho Sci
Copyright (C) 2013 Yegor Alexeyev

This binding is licensed under the same license of MeCab.
*/
#![comment = "MeCab bindings for Rust"]
#![license = "GPL/LGPL/BSD"]
#![crate_type = "lib"]

#![feature(unsafe_destructor, globs, phase)]
#![allow(non_snake_case, dead_code)]

#[cfg(test)] extern crate debug;
#[phase(plugin)] extern crate "link-config" as link_config;
extern crate libc;
extern crate encoding;

use std::string::raw::from_buf;
use std::slice::raw::buf_as_slice;
use libc::types::os::arch::c95::c_char;
use libc::types::os::arch::c95::c_int;
use libc::types::os::arch::c95::c_uint;
use libc::types::os::arch::c95::c_long;
use libc::types::os::arch::c95::c_float;
use libc::types::os::arch::c95::size_t;
use libc::funcs::c95::string::strlen;

#[cfg(test)] mod test;

link_config!("mecab")
extern "C" {
    fn mecab_new(argc: c_int, argv: *mut *mut c_char) -> *mut mecab_t;
    fn mecab_new2(arg: *const c_char) -> *mut mecab_t;
    fn mecab_destroy(mecab: *mut mecab_t);
    fn mecab_strerror(mecab: *mut mecab_t) -> *const c_char;
    fn mecab_do(argc: c_int, argv: *mut *mut c_char) -> c_int;
    fn mecab_sparse_tostr(mecab: *mut mecab_t, input: *const c_char) -> *const c_char;
    fn mecab_sparse_tostr2(mecab: *mut mecab_t, input: *const c_char, len: size_t) -> *const c_char;
    fn mecab_sparse_tonode(mecab: *mut mecab_t, input: *const c_char) -> *const mecab_node_t;
    fn mecab_sparse_tonode2(mecab: *mut mecab_t, input: *const c_char, len: size_t) -> *const mecab_node_t;
    fn mecab_parse_lattice(mecab: *mut mecab_t, lattice: *mut mecab_lattice_t) -> c_int;
    fn mecab_dictionary_info(mecab: *mut mecab_t) -> *const mecab_dictionary_info_t;
    fn mecab_version() -> *const c_char;

    fn mecab_model_new(argc: c_int, argv: *mut *mut c_char) -> *mut mecab_model_t;
    fn mecab_model_new2(arg: *const c_char) -> *mut mecab_model_t;
    fn mecab_model_new_tagger(model: *mut mecab_model_t) -> *mut mecab_t;
    fn mecab_model_new_lattice(model: *mut mecab_model_t) -> *mut mecab_lattice_t;
    fn mecab_model_destroy(model: *mut mecab_model_t);
    fn mecab_model_dictionary_info(model: *mut mecab_model_t) -> *const mecab_dictionary_info_t;

    fn mecab_lattice_set_sentence2(lattice: *mut mecab_lattice_t, input: *const c_char, len: size_t);
    fn mecab_lattice_tostr(lattice: *mut mecab_lattice_t) -> *const c_char;
    fn mecab_lattice_get_size(lattice: *mut mecab_lattice_t) -> size_t;
    fn mecab_lattice_get_bos_node(lattice: *mut mecab_lattice_t) -> *mut mecab_node_t;
    fn mecab_lattice_get_eos_node(lattice: *mut mecab_lattice_t) -> *mut mecab_node_t;
    fn mecab_lattice_get_begin_nodes(lattice: *mut mecab_lattice_t, pos: size_t) -> *mut mecab_node_t;
    fn mecab_lattice_get_end_nodes(lattice: *mut mecab_lattice_t, pos: size_t) -> *mut mecab_node_t;
    fn mecab_lattice_destroy(lattice: *mut mecab_lattice_t);
    fn mecab_lattice_strerror(lattice: *mut mecab_lattice_t) -> *const c_char;
}

#[repr(C)]
struct mecab_t;

impl mecab_t {
    unsafe fn dictionary_info(&mut self) -> *const mecab_dictionary_info_t {
        let dict = mecab_dictionary_info(&mut *self);
        if dict.is_null() {
            fail!(from_buf(mecab_strerror(&mut *self) as *const u8))
        }
        dict
    }
}

#[repr(C)]
struct mecab_model_t;

impl mecab_model_t {
    unsafe fn dictionary_info(&mut self) -> *const mecab_dictionary_info_t {
        let dict = mecab_model_dictionary_info(&mut *self);
        if dict.is_null() {
            fail!(from_buf(mecab_strerror(std::ptr::mut_null()) as *const u8))
        }
        dict
    }
}

#[repr(C)]
struct mecab_lattice_t;

/**
Same structure of `mecab::mecab_path_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__path__t.html>
*/
#[repr(C)]
struct mecab_path_t {
    rnode: *mut mecab_node_t,
    rnext: *mut mecab_path_t,
    lnode: *mut mecab_node_t,
    lnext: *mut mecab_path_t,
    cost:   c_int,
    prob:   c_float,
}

/**
Same structure of `mecab::mecab_node_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__node__t.html>
*/
#[repr(C)]
struct mecab_node_t {
    prev:      *mut mecab_node_t,
    next:      *mut mecab_node_t,
    enext:     *mut mecab_node_t,
    bnext:     *mut mecab_node_t,
    rpath:     *mut mecab_path_t,
    lpath:     *mut mecab_path_t,
    surface:   *const c_char,
    feature:   *const c_char,
    id:         c_uint,
    length:     u16,
    rlength:    u16,
    rcAttr:     u16,
    lcAttr:     u16,
    posid:      u16,
    char_type:  u8,
    stat:       u8,
    isbest:     u8,
    alpha:      c_float,
    beta:       c_float,
    prob:       c_float,
    wcost:      i16,
    cost:       c_long,
}

/**
Same structure of `mecab::mecab_dictionary_info_t` that documented in
<http://mecab.sourceforge.net/doxygen/structmecab__dictionary__info__t.html>
*/
#[repr(C)]
struct mecab_dictionary_info_t {
    filename: *const c_char,
    charset:  *const c_char,
    size:      c_uint,
    ty:        c_int,
    lsize:     c_uint,
    rsize:     c_uint,
    version:   u16,
    next:     *const mecab_dictionary_info_t,
}

/**
Parameters for `mecab_node_t.stat` Normal node
defined in the dictionary.
*/
static NOR_NODE: u8 = 0u8;

/**
Parameters for `mecab_node_t.stat` Unknown node
not defined in the dictionary.
*/
static UNK_NODE: u8 = 1u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a beginning of the sentence.
*/
static BOS_NODE: u8 = 2u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a end of the sentence.
*/
static EOS_NODE: u8 = 3u8;

/**
Parameters for `mecab_node_t.stat` Virtual node
representing a end of the N-best enumeration.
*/
static EON_NODE: u8 = 4u8;

#[repr(u8)]
pub enum Status {
    Normal = NOR_NODE,
    Unknown = UNK_NODE,
    BeginningOfSentence = BOS_NODE,
    EndOfSentence = EOS_NODE,
    EndOfNthBest = EON_NODE
}

/// Wrapped structure for `mecab_dictionary_info_t`.
pub struct DictionaryInfo {
    dict: *const mecab_dictionary_info_t
}

pub struct DictionaryInfoIterator {
    position: *const mecab_dictionary_info_t
}

impl Iterator<DictionaryInfo> for DictionaryInfoIterator {
    fn next(&mut self) -> Option<DictionaryInfo> {
        let r = self.position;
        if r.is_null() {
            None
        } else {
            unsafe { self.position = (*self.position).next; }
            Some(DictionaryInfo { dict: r })
        }
    }
}

impl DictionaryInfo {
    pub fn iter(&self) -> DictionaryInfoIterator {
        DictionaryInfoIterator { position: self.dict }
    }
}

//TODO TaggerNode and LatticeNode should expose the common interface

pub struct TaggerNode< 'tagger_owner> {
    owner: &'tagger_owner Tagger<'tagger_owner>, 
    string_data: Vec<u8>,
    node: *mut mecab_node_t, encoding: encoding::EncodingRef
}

pub struct LatticeNode<'lattice_owner> {
    owner: &'lattice_owner Lattice<'lattice_owner>, 
    node: *mut mecab_node_t, encoding: encoding::EncodingRef
}

pub struct Node {
    data: *mut mecab_node_t, encoding: encoding::EncodingRef
}

pub struct NodeIterator {
    position: *mut mecab_node_t, encoding: encoding::EncodingRef
}

impl Iterator<Node> for NodeIterator {
    fn next(&mut self) -> Option<Node> {
        let r = self.position;
        if r.is_null() || r == unsafe { (*r).enext } {
            None
        } else {
          unsafe { self.position = (*self.position).next; }
          Some(Node { data: r, encoding: self.encoding })
        }
    }
}

impl DoubleEndedIterator<Node> for NodeIterator {
    fn next_back(&mut self) -> Option<Node> {
        if self.position.is_null() { None }
        else {
            let current_position = self.position;
            unsafe { self.position = (*self.position).prev }
            Some(Node { data: current_position, encoding: self.encoding })
        }
    }
}

impl<'tagger_owner> TaggerNode< 'tagger_owner> {
    pub fn iter(&self) -> NodeIterator {
        NodeIterator { position: self.node, encoding: self.encoding }
    }
}

impl<'tagger_owner> LatticeNode<'tagger_owner> {
    pub fn iter(&self) -> NodeIterator {
        NodeIterator { position: self.node, encoding: self.encoding }
    }
}


///
/// Wrapped structure for `mecab_t`.
pub struct Tagger<'owner> {
    owner: Option<&'owner Model>,
    mecab: *mut mecab_t,
    encoding: ::encoding::EncodingRef
}

/// Wrapped structure for `mecab_model_t`.
pub struct Model {
    model: *mut mecab_model_t,
    encoding: ::encoding::EncodingRef
}

/// Wrapped structure for `mecab_lattice_t`.
pub struct Lattice<'owner> {
    owner: &'owner Model,
    lattice: *mut mecab_lattice_t,
    sentence: Vec<u8>
}


impl Drop for DictionaryInfo {
    fn drop(&mut self) {}
}

#[unsafe_destructor]
impl<'tagger_owner> Drop for TaggerNode<'tagger_owner> {
    fn drop(&mut self) {}
}

#[unsafe_destructor]
impl<'tagger_owner> Drop for LatticeNode<'tagger_owner> {
    fn drop(&mut self) {}
}

#[unsafe_destructor]
impl<'owner> Drop for Tagger<'owner> {
    fn drop(& mut self) {
        unsafe { mecab_destroy(self.mecab); }
    }
}

impl Drop for Model {
    fn drop(&mut self) {
        unsafe { mecab_model_destroy(self.model); }
    }
}

#[unsafe_destructor]
impl<'owner> Drop for Lattice<'owner> {
    fn drop(&mut self) {
        unsafe { mecab_lattice_destroy(self.lattice); }
    }
}

impl DictionaryInfo {
    /// Returns `mecab_dictionary_info_t.filename`.
    pub fn get_filename(&self) -> String {
        unsafe { from_buf((*self.dict).filename as *const u8) }
    }

    /// Returns `mecab_dictionary_info_t.charset`.
    pub fn get_charset(&self) -> String {
        unsafe { from_buf((*self.dict).charset as *const u8) }
    }

    /// Returns `mecab_dictionary_info_t.size`.
    pub fn get_size(&self) -> uint {
       unsafe { (*self.dict).size as uint }
    }

    /// Returns `mecab_dictionary_info_t.type`.
    pub fn get_type(&self) -> int {
        unsafe { (*self.dict).ty as int }
    }

    /// Returns `mecab_dictionary_info_t.lsize`.
    pub fn get_lsize(&self) -> uint {
        unsafe { (*self.dict).lsize as uint }
    }

    /// Returns `mecab_dictionary_info_t.rsize`.
    pub fn get_rsize(&self) -> uint {
        unsafe { (*self.dict).rsize as uint }
    }

    /// Returns `mecab_dictionary_info_t.version`.
    pub fn get_version(&self) -> uint {
        unsafe { (*self.dict).version as uint }
    }
}

impl Node {
    pub fn get_surface(&self) -> String {
        unsafe {
            buf_as_slice(
                (*self.data).surface, (*self.data).length as uint,
                |v| decode(self.encoding, v))
        }
    }

    /// Returns `mecab_node_t.feature`.
    pub fn get_feature(&self) -> String {
        unsafe {
            buf_as_slice(
                (*self.data).feature, strlen((*self.data).feature) as uint,
                |v| decode(self.encoding, v))
        }
    }

    /// Returns `mecab_node_t.status`.
    pub fn get_status(&self) -> Status {
        match unsafe { (*self.data).stat } {
            NOR_NODE => Normal,
            UNK_NODE => Unknown,
            BOS_NODE => BeginningOfSentence,
            EOS_NODE => EndOfSentence,
            EON_NODE => EndOfNthBest,
            _ => unreachable!()
        }
    }

    /// Returns `mecab_node_t.posid`.
    pub fn get_posid(&self) -> u16 {
        unsafe { (*self.data).posid }
    }

    /// Returns `mecab_node_t.prob`.
    pub fn get_prob(&self) -> c_float {
        unsafe { (*self.data).prob }
    }

    pub fn is_best(&self) -> bool {
        unsafe { (*self.data).isbest == 1 }
    }
}

fn encoding_from_name(n: &str) -> encoding::EncodingRef {
    if n == "euc-jp" {
        encoding::all::EUC_JP
    } else if n == "utf8" {
        encoding::all::UTF_8
    } else {
        fail!("unknown encoding: {}", n)
    }
}

fn encode(encoding: encoding::EncodingRef, s: &str) -> Vec<u8> {
    let mut r = encoding.encode(s, encoding::EncodeReplace).unwrap();
    r.push(0);
    r
}

fn decode(encoding: encoding::EncodingRef, s: &[c_char]) -> String {
    unsafe {
        buf_as_slice(s.as_ptr() as *const u8, s.len(), |v|
                     encoding.decode(v, encoding::DecodeReplace).unwrap())
    }
}

impl<'owner> Tagger<'owner> {

    /// The wrapper of `mecab::mecab_new` that may return `Tagger`.
    pub fn new(args: &[Box<str>]) -> Tagger {
        let argc = args.len() as c_int;

        let mut argptrs = Vec::new();
        let mut tmps = Vec::new();

        for arg in args.iter() {
            //let t = @copy *arg;
            let t = (*arg).clone();
            tmps.push(t.clone());
            argptrs.push(t.as_ptr());
        }
        argptrs.push(::std::ptr::null());

        let mecab = unsafe { mecab_new( argc, argptrs.as_ptr() as *mut *mut c_char ) };

        if mecab.is_null() {
            fail!("failed to create new instance");
        } else {
            Tagger { mecab: mecab, owner: None,
                     encoding: encoding_from_name(DictionaryInfo {
                         dict: unsafe { (*mecab).dictionary_info() }
                     }.get_charset().as_slice()) }
        }
    }

    /// The wrapper of `mecab::mecab_new2` that may return `Tagger`.
    pub fn new2(arg: &str) -> Tagger {
        let mecab = unsafe { mecab_new2(arg.as_ptr() as *const c_char) };

        if mecab.is_null() {
            fail!("failed to create new instance {}",
                  unsafe { from_buf(mecab_strerror(std::ptr::mut_null()) as *const u8) });
        } else {
            Tagger { mecab: mecab, owner: None,
                     encoding: encoding_from_name(DictionaryInfo {
                         dict: unsafe { (*mecab).dictionary_info() }
                     }.get_charset().as_slice())}
        }
    }

    /// Parses input and may return the string of result.
    pub fn parse(&self, input: &str) -> String {
        let s = unsafe {
            mecab_sparse_tostr(self.mecab, encode(self.encoding, input).as_ptr() as *const c_char)
        };

        if s.is_null() {
            let msg = self.strerror();
            fail!(msg);
        } else {
            unsafe { buf_as_slice(s, strlen(s) as uint, |v| decode(self.encoding, v)) }
        }
    }

    /// Parses input and returns `Node`.
    pub fn parse_to_node<'tagger>(&'tagger self, input: &str) -> TaggerNode<'tagger> {
        unsafe {
            let input = encode(self.encoding, input);
            let node = mecab_sparse_tonode2(self.mecab, input.as_ptr() as *const c_char,
                                            strlen(input.as_ptr() as *const c_char));
            if node.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                TaggerNode { owner: self, node: node as *mut mecab_node_t,
                             string_data: input, encoding: self.encoding }
            }
        }
    }

    /// Parses input in given `lattice` and returns true on success.
    pub fn parse_lattice(&self, lattice: &Lattice) -> bool {
        unsafe {
            let status = mecab_parse_lattice(self.mecab, lattice.lattice);
            status != 0 as c_int
        }
    }

    /// Returns `DictionaryInfo`.
    pub fn get_dictionary_info(&self) -> DictionaryInfo {
        unsafe {
            let dict = mecab_dictionary_info(self.mecab);

            if dict.is_null() { fail!(self.strerror()); }
            DictionaryInfo { dict: dict }
        }
    }

    pub fn strerror(&self) -> String {
        unsafe {
            let s = mecab_strerror(self.mecab);
            from_buf(s as *const u8)
        }
    }
}

impl Model {

    /**

    The wrapper of `mecab::mecab_model_new` that
    may return `Model`.
    */
    pub fn new(args: &[Box<str>]) -> Model {
        let argc = args.len() as c_int;

        let mut argptrs = Vec::new();
        let mut tmps = Vec::new();

        for arg in args.iter() {
            let t = (*arg).clone();
            tmps.push(t.clone());
    //TODO I'm not sure
            argptrs.push(t.as_ptr());
        }
        argptrs.push(::std::ptr::null());

        let model = unsafe { mecab_model_new( argc, argptrs.as_ptr() as *mut *mut c_char ) };

        if model.is_null() {
            fail!("failed to create new Model");
        } else {
            Model { model: model,
                    encoding: encoding_from_name(DictionaryInfo {
                        dict: unsafe { (*model).dictionary_info() }
                    }.get_charset().as_slice()) }
        }
    }

    /**
    The wrapper of `mecab::mecab_model_new2` that
    may return `Model`.
    */
    pub fn new2(arg: &str) -> Model {
        let model = unsafe { mecab_model_new2(arg.as_ptr() as *const i8) };

        if model.is_null() {
            fail!("failed to create new Model");
        } else {
            Model { model: model,
                    encoding: encoding_from_name(DictionaryInfo {
                        dict: unsafe { (*model).dictionary_info() }
                    }.get_charset().as_slice()) }
        }
    }

    /// Creates new tagger.
    pub fn create_tagger<'model>(&'model self) -> Tagger<'model> {
        unsafe {
            let mecab = mecab_model_new_tagger(self.model);

            if mecab.is_null() {
                fail!("failed to create new Tagger");
            } else {
                Tagger {owner: Some(self), mecab: mecab,
                        encoding: encoding_from_name(DictionaryInfo {
                            dict: (*mecab).dictionary_info()
                        }.get_charset().as_slice())}
            }
        }
    }

    /// Creates new lattice.
    pub fn create_lattice<'model>(&'model self) -> Lattice<'model> {
        unsafe {
            let lattice = mecab_model_new_lattice(self.model);

            if lattice.is_null() {
                fail!("failed to create new Lattice");
            } else {
                Lattice { owner: self, lattice: lattice, sentence: vec!() }
            }
        }
    }
}

impl<'owner> ::std::fmt::Show for Lattice<'owner> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::FormatError> {
        unsafe {
            let s = mecab_lattice_tostr(self.lattice);
            match f.write_str(buf_as_slice(s, strlen(s) as uint, |v|
                                           decode(self.owner.encoding, v)).as_slice()) {
                Err(_) => Err(::std::fmt::WriteError), _ => Ok(()) }
        }
    }
}

impl<'owner> Lattice<'owner> {
    /// Set input of the lattice.
    pub fn set_sentence(&mut self, input: &str) {
        unsafe {
            let bytes = encode(self.owner.encoding, input);
            mecab_lattice_set_sentence2(self.lattice, bytes.as_ptr() as *const c_char, (bytes.len() - 1) as size_t); 
            self.sentence = bytes;
        }
    }

    /// Returns the beginning node of the sentence on success.
    pub fn get_bos_node<'lattice>(&'lattice self) -> LatticeNode<'lattice> {
        unsafe {
            let node = mecab_lattice_get_bos_node(self.lattice);

            if node.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                LatticeNode { owner: self, node: node, encoding: self.owner.encoding }
            }
        }
    }

    /// Returns the end node of the sentence on success.
    fn get_eos_node<'lattice>(&'lattice self) -> LatticeNode<'lattice> {
        unsafe {
            let node = mecab_lattice_get_eos_node(self.lattice);

            if node.is_null() {
                let msg = self.strerror();
                fail!(msg);
            } else {
                LatticeNode { owner: self, node: node, encoding: self.owner.encoding }
            }
        }
    }

    fn strerror(&self) -> String {
        unsafe {
            let s = mecab_lattice_strerror(self.lattice);
            from_buf(s as *const u8)
        }
    }

    fn get_size(&self) -> size_t { unsafe { mecab_lattice_get_size(self.lattice) } }
}



/**
The wrapper of `mecab::mecab_version` that
returns version-number string.
*/
pub fn version() -> String {
    unsafe {
        let vers = mecab_version();
        from_buf(vers as *const u8)
    }
}
