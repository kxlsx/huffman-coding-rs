use std::{
    cell::RefCell, 
    cmp::Ordering,
};

use crate::{HuffLeaf, HuffCode};



/// Struct representing a node in the Huffman Tree.
/// 
/// Stores its children as:
/// 
/// [Option<Box<RefCell<HuffBranch>>>; 2]
/// 
/// Also stores its position in the parent's children Array, and 
/// data represented as a HuffLeaf.
#[derive(Debug, Clone, Eq)]
pub struct HuffBranch{
    leaf: HuffLeaf,

    pos_in_parent: Option<u8>,
    children: Option<[Box<RefCell<HuffBranch>>; 2]>
}

impl Ord for HuffBranch {
    fn cmp(&self, other: &Self) -> Ordering {
        other.leaf().frequency().cmp(&self.leaf().frequency())
    }
}

impl PartialOrd for HuffBranch {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffBranch {
    fn eq(&self, other: &Self) -> bool {
        self.leaf().frequency() == other.leaf().frequency()
    }
}

impl HuffBranch{
    /// Initializes a new HuffBranch.
    /// 
    /// # Example
    /// ---
    /// ```
    /// use huff_encoding::{HuffBranch, HuffLeaf};
    /// 
    /// let foo = HuffBranch::new(HuffLeaf::new(Some(0xc4), 3), None);
    /// ```
    pub fn new(leaf: HuffLeaf, children: Option<[Box<RefCell<HuffBranch>>; 2]>) -> Self{
        HuffBranch{
            leaf,

            pos_in_parent: None,
            children
        }
    }

    /// Returns a reference to the stored HuffLeaf.
    pub fn leaf(&self) -> &HuffLeaf{
        &self.leaf
    }

    /// Returns its position in the parent's children Array
    pub fn pos_in_parent(&self) -> Option<u8>{
        self.pos_in_parent
    }

    /// Returns the stored Array of the branch's children
    pub fn children(&self) -> Option<&[Box<RefCell<HuffBranch>>; 2]>{
        match self.children{
            None => 
                None,
            Some(_) => {
                self.children.as_ref()
            }
        }
    }


    /// Sets the given children array
    pub fn set_children(&mut self, children: Option<[Box<RefCell<HuffBranch>>; 2]>){
        self.children = children;
    }

    /// Sets the stored position in parent's children Array
    pub fn set_pos_in_parent(&mut self, pos_in_parent: u8){
        self.pos_in_parent = Some(pos_in_parent);
    } 

    /// Sets its leaf's code based on the given parent_code and its
    /// pos_in_parent.
    pub fn set_code(&mut self, parent_code: Option<&HuffCode>){
        let mut code = HuffCode::new();

        if self.pos_in_parent().is_some(){
            if parent_code.is_some(){
                for bit in parent_code.unwrap(){
                    code.push(bit);
                }
            }
            
            code.push(self.pos_in_parent().unwrap() >= 1);

            self.leaf.set_code(code);
        }
    }
}
