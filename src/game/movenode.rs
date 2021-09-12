use std::ops::Sub;

use super::r#move::*;

#[derive(Clone)]
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
        self.nodes[parent_index].children_index = Vec::new();
    }

    pub fn orphan_siblings(&mut self, favoured_child_index: usize) {
        if favoured_child_index == self.root_index { return; } // if this is root, DON'T!

        let favoured_child = self.nodes[favoured_child_index].clone();
        let parent_index = favoured_child.parent_index;
    
        self.orphan_children(parent_index);
        self.add_child_to(favoured_child, parent_index);
    }
    
    pub fn orphan_siblings_recursive(&mut self, favoured_child_index: usize) {
        if favoured_child_index == self.root_index { return; } // if this is root, DON'T!

        let favoured_child = self.nodes[favoured_child_index].clone();
        let parent_index = favoured_child.parent_index;
    
        self.orphan_children(parent_index);
        self.add_child_to(favoured_child, parent_index);

        // recursion
        self.orphan_siblings_recursive(parent_index);
    }

    pub fn clone_without_orphans(&mut self) -> Self{
        let mut new_tree = Self::new();
        new_tree.clone_without_orphans_inner(&self, new_tree.root_index, self.root_index);
        new_tree
    }

    fn clone_without_orphans_inner(&mut self, other: &Self, parent_index: usize, other_index: usize) {
        // Recursively clones other into self (ignoring orphans).
        for other_child_index in other.nodes[other_index].children_index.iter() {
            let new_child_index = self.add_child_to(other.nodes[*other_child_index].clone(), parent_index);
            self.clone_without_orphans_inner(other, new_child_index, other_index);
        } 
    }
}

#[derive(Clone)]
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
