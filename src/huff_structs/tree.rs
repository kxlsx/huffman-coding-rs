#![allow(dead_code)]


use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
use std::collections::HashMap;
use crate::huff_structs::{HuffBranch, HuffLeaf, chars_to_freq};
use crate::huff_structs::branch_heap::HuffBranchHeap;



/// Struct representing a Huffman Tree.
/// 
/// A HuffTree is comprised of HuffBranches, each having
/// 2 or 0 children, with root being the top one and 
/// every bottom one containing a char.
/// 
/// Can be grown from: 
/// ```
/// HashMap<char, usize>
/// ```
/// or 
/// ```
/// &str
/// ```
/// or even initialized empty and grown afterwards.
/// 
#[derive(Debug)]
pub struct HuffTree{
    root: Option<Rc<RefCell<HuffBranch>>>,
    char_codes: HashMap<char, String>,
}

impl HuffTree{
    pub fn from(s: &str) -> HuffTree{
        //! Creates a HuffTree from:
        //! ```
        //! &str
        //! ```
        //! 
        //! # Example
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffTree;
        //! 
        //! let foo = HuffTree::from("Hello, World!");
        //! ```


        let mut huff_tree = HuffTree::new(None);
        huff_tree.grow(s);

        return huff_tree
    } 

    pub fn from_ctf(ctf: &HashMap<char, usize>) -> HuffTree{
        //! Creates a HuffTree from:
        //! ```
        //! HashMap<char, usize>
        //! ```
        //! 
        //! # Example
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::{HuffTree, get_chars_to_freq};
        //! 
        //! let foo = HuffTree::from(get_chars_to_freq("Hello, World!"));
        //! ```


        let mut huff_tree = HuffTree::new(None);
        huff_tree.grow_ctf(ctf);

        return huff_tree;
    }

    pub fn new(root: Option<Rc<RefCell<HuffBranch>>>) -> HuffTree{
        //! Initializes a HuffTree with the given root.
        //! 
        //! Can be grown later with .grow or .grow_ctf
        //! 
        //! # Example
        //! ```
        //! use huff_encoding::huff_structs::HuffTree;
        //! 
        //! let foo = HuffTree::new();
        //! foo.grow("Hello, World!");
        //! ```


        let huff_tree = HuffTree{
            root: root,
            char_codes: HashMap::new(),
        };

        return huff_tree;
    }

    
    pub fn root(&self) -> Option<&Rc<RefCell<HuffBranch>>>{
        //! Returns the root of the tree.
        

        match self.root{
            Some(_) =>
                return self.root.as_ref(),
            None =>
                return None,
        }
    }

    pub fn char_codes(&self) -> &HashMap<char, String>{
        //! Returns a HashMaps of chars with their
        //! corresponding Huffman codes.


        return &self.char_codes;
    }


    pub fn as_bin(&self) -> String{
        //! Returns the tree represented in binary
        //! to be stored as a header to an encoded file:
        //! 
        //! * 0 being a character leaf
        //! ..* after a 0 you can expect a 32b char.
        //! * 1 being a joint leaf.
        //! 
        //! ---
        //! ## DOES NOT STORE FREQUENCIES.
        //! It's only meant to construct a same
        //! shaped tree for decoding a file.
        //! 
        //! ---
        //! 
        //! # Examples
        //! ---
        //! ```
        //! use huff_encoding::huff_structs::HuffTree;
        //! 
        //! let foo = HuffTree::from("abbccc");
        //! 
        //! // does not panic
        //! assert!(&foo.as_bin()[..] == 
        //!     "10000000000000000000000000011000111000000000000000000000000001100001000000000000000000000000001100010")
        //! ```

        let mut bin = String::new();
        HuffTree::set_bin(&mut bin, self.root().unwrap().borrow());

        return bin;
    }


    pub fn grow(&mut self, s: &str){
        //! Grows the tree from the given:HuffTree
        //! ```
        //! &str
        //! ```


        assert!(s.len() > 0, "slice is empty");
        self.grow_ctf(&chars_to_freq(s));
    }

    pub fn grow_ctf(&mut self, ctf: &HashMap<char, usize>){
        //! Grows the tree from the given:HuffTree
        //! ```
        //! &HashMap<char, usize>
        //! ```


        assert!(ctf.len() > 0, "ctf is empty");

        let mut branch_heap = HuffBranchHeap::from(&ctf);


        while branch_heap.len() > 1{
            let mut min = branch_heap.pop_min();
            let mut next_min = branch_heap.pop_min();

            min.set_pos_in_parent(0);
            next_min.set_pos_in_parent(1);

            let branch = HuffBranch::new(
                HuffLeaf::new(
                    None,
                    min.leaf().frequency() + next_min.leaf().frequency()
                ),
                [Some(Rc::new(RefCell::new(min))), Some(Rc::new(RefCell::new(next_min)))]
            );
            branch_heap.push(branch);
        }

        let root = Some(Rc::new(RefCell::new(branch_heap.pop_min())));
        self.root = root;

        // set codes for all branches recursively
        HuffTree::set_branch_codes(self.root().unwrap().borrow_mut());

        // set chard_codes recursively
        let mut char_codes: HashMap<char, String> = HashMap::new();
        self.set_char_codes(&mut char_codes, self.root().unwrap().borrow());
        self.char_codes = char_codes;
    }


    fn set_branch_codes(root: RefMut<HuffBranch>){
        let root = root;
        let children = root.children();

        match children{
            [Some(_), Some(_)] =>{
                let root_code = root.leaf().code();
                for child in children.iter(){
                    child.unwrap().borrow_mut().set_code(root_code);
                    HuffTree::set_branch_codes(child.unwrap().borrow_mut());
                }
            }
            _ =>
                (),
        }
    }

    fn set_char_codes(&self, char_codes: &mut HashMap<char, String>, root: Ref<HuffBranch>){
        let root = root;
        let children = root.children();

        match children{
            [Some(_), Some(_)] =>{   
                for child in children.iter(){
                    let branch = child.unwrap().borrow();
                    let leaf = branch.leaf();
                    let c = leaf.character();
                    match c{
                        Some(_) =>{
                            char_codes.insert(c.unwrap(), leaf.code().unwrap().clone());
                        }
                        None =>{
                            self.set_char_codes(char_codes, child.unwrap().borrow());
                        }
                    }
                }
            }
            _ =>{
                char_codes.insert(root.leaf().character().unwrap(), String::from("0"));
            }
        }

    }

    fn set_bin(bin: &mut String, root: Ref<HuffBranch>){
        let root = root;
        let children = root.children();

        match children{
            [Some(_), Some(_)] =>{
                bin.push('1');
                for child in children.iter(){
                    HuffTree::set_bin(bin, child.unwrap().borrow());
                }
            }
            _ =>{
                bin.push('0');
                bin.push_str(&format!("{:032b}", root.leaf().character().unwrap() as u32));
            }
        }
    }
}