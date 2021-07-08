use std::ops::Sub;

use super::r#move::*;

pub struct Movetree {
    nodes: Vec<Movenode>,
    root_index: usize,
}

impl Movetree {
    pub fn new() -> Self{
        Self {
            nodes: Vec::from([Movenode::new_root()]),
            root_index: 0,
        }
        
    }

    pub fn add_child_to(&mut self, mut child: Movenode, parent_index: usize) -> usize {
        assert!(parent_index < self.nodes.len(), "Parant index oob!");
        child.parent_index = parent_index;
        self.nodes.push(child);
        let child_index: usize = self.nodes.len() - 1;
        self.nodes[parent_index].children_index.push(child_index);

        return child_index
    }

    pub fn orphan_children(&mut self, parent_index: usize) {
        self.nodes[parent_index].children_index = []
    }

    pub fn orphan_siblings(&mut self, favourd_child_index: usize) {
        if favourd_child_index == self.root_index { return; } // if this is root, DON'T!

        let favourd_child = self.nodes[favourd_child_index];
        let parent_index = favourd_child.parent_index;
    
        self.orphan_children(parent_index);
        self.add_child_to(favourd_child, parent_index);
    }
    
    pub fn orphan_siblings_recursive(&mut self, favourd_child_index: usize) {
        if favourd_child_index == self.root_index { return; } // if this is root, DON'T!

        let favourd_child = self.nodes[favourd_child_index];
        let parent_index = favourd_child.parent_index;
    
        self.orphan_children(parent_index);
        self.add_child_to(favourd_child, parent_index);

        // recursion
        self.orphan_siblings_recursive(parent_index);
    }

    pub fn clone_without_orphans(&mut self, other: Self) -> Self{
        let mut new_tree = Self::new();
        new_tree.copy_family_recursive(&mut self, new_tree.root_index, self.root_index);
        new_tree
    }

    pub fn copy_family_recursive(&mut self, other: &mut Self, parent_index: usize, other_index: usize) {
        for other_child_index in other.nodes[other_index].children_index.iter() {
            let new_child_index = self.add_child_to(other.nodes[*other_child_index], parent_index);
            self.copy_family_recursive(other, new_child_index, other_index);
        } 
    }
}

pub struct Movenode {
    parent_index: usize,
    submove: Submove,
    children_index: Vec<usize>,
}

impl Movenode {
    pub fn new_root() -> Self{
        Self {
            parent_index: 0,
            submove: Submove::new_debug(), // PLACEHOLDER!!
            children_index: Vec::new(),
        }
    }
}
