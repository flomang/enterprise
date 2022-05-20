pub mod utils;
pub mod auth;
pub mod db;
pub mod errors;
pub mod pagination;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
