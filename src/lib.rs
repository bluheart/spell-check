mod utils;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


type Alpha = Box<HashMap<char, Node>>;

#[wasm_bindgen]
pub struct Node {
    terminal: bool,
    alpha: Alpha
}

#[wasm_bindgen]
pub struct Trie {
    root: Node
}

#[wasm_bindgen]
impl Trie {

    pub fn new() -> Self {
        Trie { root: Node {terminal: false, alpha: Box::new(HashMap::new())} }
    }

    pub fn load(&mut self, file: String) {
        let words = file.lines();
        for word in words {
            self.new_word(word.trim().to_string());
        }
    }

    //uses a depth first search to retrieve all
    //words with the given prefix
    pub fn auto_complete(self, prefix: String) {

    }

    //loads a word into Trie
    pub fn new_word(&mut self, word: String) {
        let mut iter = &mut self.root;
        for c in word.chars() {
            if iter.alpha.contains_key(&c) {
                iter = iter.alpha.get_mut(&c).unwrap();
            }
            else {
                iter.alpha.insert(c, Node {terminal: false, alpha: Box::new(HashMap::new())});
                iter = iter.alpha.get_mut(&c).unwrap();
            }
        }
        iter.terminal = true;
    }

    //search Trie for word
    pub fn search(&mut self, word: String) -> bool {
        let mut iter: &mut Node = &mut self.root;
        for c in word.chars() {
            if iter.alpha.contains_key(&c) {
                iter = iter.alpha.get_mut(&c).expect("couldn't get node");
            }
            else { return false }
        }
        return iter.terminal
    }
}
