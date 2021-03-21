mod type_uuid;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 2, 4);
    }
}

pub use type_uuid::*;
pub use reflect_derive::TypeUuid;
