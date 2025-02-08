// serde generated deserialization types for yosys json output
// these are designed for maximum lazy compatibility with yosys,
// and are not at all memory efficient
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Design {
    modules: HashMap<String, Module>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Module {
    attributes: HashMap<String, String>,
    ports: HashMap<String, Port>,
    cells: HashMap<String, Cell>,
    netnames: HashMap<String, Net>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Direction {
    #[serde(rename = "input")]
    Input,
    #[serde(rename = "output")]
    Output,
    #[serde(rename = "inout")]
    Inout,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Signal {
    Constant(String),
    // #[serde(rename = "0")]
    // Zero,
    // #[serde(rename = "1")]
    // One,
    // #[serde(rename = "x")]
    // X,
    // #[serde(rename = "z")]
    // Z,
    Net(u32),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Port {
    direction: Direction,
    bits: Vec<Signal>,
    offset: Option<u32>,
    upto: Option<u32>,
    signed: Option<u8>,
}

fn deserialize_bitstring<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    u32::from_str_radix(s.as_str(), 2).map_err(serde::de::Error::custom)
}

fn deserialize_bitboolean<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    u32::from_str_radix(s.as_str(), 2)
        .map(|i| i == 1)
        .map_err(serde::de::Error::custom)
}

fn deserialize_signal<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    u32::from_str_radix(s, 2).map_err(serde::de::Error::custom)
}

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(tag = "type")]
// enum Cell {
//     #[serde(rename = "$and")]
//     And {
//         hide_name: u8,
//         port_directions: AndDirections,
//     },
// }

#[derive(Serialize, Deserialize, Debug)]
enum CellType {
    #[serde(rename = "$and")]
    And,
    #[serde(rename = "$eq")]
    Eq,
    #[serde(rename = "$or")]
    Or,
    #[serde(rename = "$xor")]
    Xor,
    #[serde(rename = "$mux")]
    Mux,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Parameters {
    #[serde(rename = "A_SIGNED", deserialize_with = "deserialize_bitboolean")]
    a_signed: bool,
    #[serde(rename = "A_WIDTH", deserialize_with = "deserialize_bitstring")]
    a_width: u32,
    #[serde(rename = "B_SIGNED", deserialize_with = "deserialize_bitboolean")]
    b_signed: bool,
    #[serde(rename = "B_WIDTH", deserialize_with = "deserialize_bitstring")]
    b_width: u32,
    #[serde(rename = "Y_WIDTH", deserialize_with = "deserialize_bitstring")]
    y_width: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Directions {
    #[serde(rename = "A")]
    a: Direction,
    #[serde(rename = "B")]
    b: Direction,
    #[serde(rename = "Y")]
    y: Direction,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    hide_name: u8,
    #[serde(rename = "type")]
    cell_type: CellType,
    parameters: Parameters,
    port_directions: Directions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Net {
    hide_name: u8,
    bits: Vec<Signal>,
    offset: Option<u32>,
    upto: Option<u32>,
    signed: Option<u8>,
}
