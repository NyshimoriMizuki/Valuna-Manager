mod phex;
mod setupcl;
mod wordgen;

use setupcl::SetupCL;

fn main() {
    let test = SetupCL::from_json("samples/exemplish-sucl.json");
}
