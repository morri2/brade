use super::move::*;

pub enum Movenode{
    Root(Vec<Movenode>),
    Node(Submove, Vec<Movenode>),
}

impl Movenode{

}
