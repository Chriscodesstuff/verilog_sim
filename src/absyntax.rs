use rustlr::LBox;
use crate::Construct::*;

#[derive(Debug)]
pub enum Construct<'t> {
    Id(&'t str),
    Module(ModuleDec<'t>),
    Header(HeaderDec<'t>),
    Ports(Vec<LBox<PortDec<'t>>>),
    Port(PortDec<'t>),
}
impl Default for Construct<'_> {
    fn default() -> Self { Module(ModuleDec::default()) }
}

#[derive(Debug)]
pub enum Direction {
    Input,
    Output,
    Inout,
    Unspecified
}

#[derive(Debug)]
pub struct ModuleDec<'t> {
    pub name:&'t str,
    pub ports: Vec<LBox<PortDec<'t>>>,
}
impl<'t> Default for ModuleDec<'t> {
    fn default() -> Self {ModuleDec{name:"",ports:Vec::new()} }
}

#[derive(Debug)]
pub struct HeaderDec<'t> {
    pub name:&'t str,
    pub ports: Vec<LBox<PortDec<'t>>>,
}
impl<'t> Default for HeaderDec<'t> { 
    fn default() -> Self { HeaderDec{name:"",ports:Vec::new()} } 
}

#[derive(Debug)]
pub struct PortDec<'t> {
    pub name:&'t str,
    pub direction: Direction,
}
impl<'t> Default for PortDec<'t> {
    fn default() -> Self { PortDec{name:"",direction:Direction::Unspecified} } 
}
