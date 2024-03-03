//! # Save
//!
//! The different kinds of worlds in Macaw.

// there needs to be a way to make worlds without a generator
// for map/mod creators. also, custom generators are cool
// and i'll leave the option open for anyone who wants to give it
// a try!

pub enum WorldType {
    Generated(Generator),
    Empty,
}

pub enum Generator {
    Default,
    Custom(Fn),
}
