extern crate egg;

use std::{error, fmt, str};

use egg::{define_language, rewrite, Id, Symbol};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum Bit {
    Zero,
    One,
    X,
    Z,
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct BitParseError;

impl error::Error for BitParseError {}
impl fmt::Display for BitParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse bit")
    }
}

impl str::FromStr for Bit {
    type Err = BitParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Bit::Zero),
            "1" => Ok(Bit::One),
            "x" => Ok(Bit::X),
            "z" => Ok(Bit::Z),
            _ => Err(BitParseError),
        }
    }
}

define_language! {
    enum Logic {
        "not" = Not(Id),
        "and" = And([Id; 2]),
        "or" = Or([Id; 2]),
        "xor" = Xor([Id; 2]),
        "add" = Add([Id; 3]),
        "output" = Output(Box<[Id]>),

        Bit(Bit),
        Symbol(Symbol),
    }
}

fn adder(i: u32) -> String {
    format!("(output (xor (xor i_a{i} i_b{i}) i_cin{i}) (or (or (and i_a{i} i_b{i}) (and i_a{i} i_cin{i})) (and i_b{i} i_cin{i})))", i = i)
}

fn main() {
    let mut s = String::from("(output ");
    for i in 0..32 {
        s = format!("{} {}", s, adder(i));
    }
    s = format!("{})", s);

    // let s = format!(
    //     "(output {} {} {} {})",
    //     adder(0),
    //     adder(1),
    //     adder(2),
    //     adder(3)
    // );
    // let s = "(output (xor (xor i_a i_b) i_cin) (or (or (and i_a i_b) (and i_a i_cin)) (and i_b i_cin)))";
    // let s = "(or a (or b (or c d)))";
    // let s = "(output (xor (xor i_a i_b) i_cin) (or (and (xor i_a i_b) i_cin) (and i_a i_b)))";
    let expr: egg::RecExpr<Logic> = s.parse().unwrap();

    println!("{:?}", expr);

    let rules: Vec<egg::Rewrite<Logic, ()>> = vec![
        rewrite!("fold"; "(and ?a 0)" => "0"),
        rewrite!("commute-and"; "(and ?a ?b)" => "(and ?b ?a)"),
        rewrite!("commute-or"; "(or ?a ?b)" => "(or ?b ?a)"),
        rewrite!("commute-xor"; "(xor ?a ?b)" => "(xor ?b ?a)"),
        rewrite!("commute-out"; "(output ?a ?b)" => "(output ?b ?a)"),
        rewrite!("assoc-and"; "(and ?a (and ?b ?c))" => "(and (and ?a ?b) ?c)"),
        rewrite!("assoc-or"; "(or ?a (or ?b ?c))" => "(or (or ?a ?b) ?c)"),
        rewrite!("assoc-xor"; "(xor ?a (xor ?b ?c))" => "(xor (xor ?a ?b) ?c)"),
        rewrite!("rewrite-xor"; "(or (and ?a ?b) (and ?c ?b))" => "(and (xor ?a ?c) ?b)"),
        rewrite!("rewrite-add"; "(output (xor (xor ?a ?b) ?c) (or (and (xor ?a ?b) ?c) (and ?a ?b)))" => "(add ?a ?b ?c)"),
    ];

    let runner = egg::Runner::default().with_expr(&expr).run(&rules);
    let root = runner.roots[0];

    let extractor = egg::Extractor::new(&runner.egraph, egg::AstSize);
    let (best_cost, best) = extractor.find_best(root);

    println!("runner stop: {:?}\n", runner.stop_reason);
    // println!("cost: {}", best_cost);
    println!("new: {}\ncost {}", best, best_cost);
    // println!("new: {:?}\ncost {}", expr, best, best_cost);

    let mut egraph = egg::EGraph::<Logic, ()>::default();
    egraph.add_expr(&best);
    // runner.egraph.dot().to_svg("graph.svg").unwrap();
    // egraph.dot().to_svg("extract.svg").unwrap();
}
