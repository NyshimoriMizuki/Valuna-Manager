mod phex;
mod setupcl;
mod wordgen;

use setupcl::SetupCL;
use wordgen::WordGenerator;

const TEST_PHEX_THINGS: bool = true;

fn main() {
    if TEST_PHEX_THINGS {
        phex::PhexReader::teste();
    }

    let setup = SetupCL::from_json("samples/exemplish-sucl.json");
    let mut generator = WordGenerator::new(&setup);

    println!("\n{:?}\n", generator.gen_words(1));
}
