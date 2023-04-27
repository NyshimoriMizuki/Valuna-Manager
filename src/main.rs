mod phex;
mod setupcl;
mod wordgen;

use setupcl::SetupCL;
use wordgen::WordGenerator;

fn main() {
    let setup = SetupCL::from_json("samples/exemplish-sucl.json");
    let mut generator = WordGenerator::new(&setup);

    setup.break_syllable_struct();

    println!("{:?}", generator.gen_words(5));
}
