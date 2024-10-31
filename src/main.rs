use pest_derive::Parser;
use pest::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result< () > {
    let got = Grammar::parse(Rule::file, "-273.0, 23, 1, -1, 7777.777\n")?;
    println!("{:?}", got);

    Ok(())
}