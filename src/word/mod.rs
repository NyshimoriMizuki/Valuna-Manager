mod gen;

pub use gen::WordGenerator;

pub struct Word {
    id: String,
    sound: String,
    class: String,
    meaning: String,
}
