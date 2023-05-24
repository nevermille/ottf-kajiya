mod avar;
mod unknown;

pub use avar::Avar;

pub struct Tables {
    pub avar: Option<Avar>,
}
