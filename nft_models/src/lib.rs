mod lemon;
mod slots;

pub use lemon::Lemon;
pub use slots::Slots;

#[enum_dispatch(Slots)]
pub enum ModelKind {
    Lemon,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
