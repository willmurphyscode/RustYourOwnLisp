use std::fmt::{Debug, Formatter, Error};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CoolAdjective {
   Horrendous,
   Stupendous,
   Gargantuan,
   Ultimate
}


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CoolNoun {
    Space,
    Time,
    Matter,
    Energy,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FinalNoun {
    Kablooie,
    Swoosh,
    Frobber,
    Atomizer,
    Event,
    Device
}

#[derive(Debug,PartialEq)]
pub struct ScientificName {
    pub adjective_list : Vec<Box<CoolAdjective>>,
    pub cool_noun : CoolNoun,
    pub final_noun : FinalNoun,
}




