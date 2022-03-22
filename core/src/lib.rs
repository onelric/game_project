extern crate num;
extern crate serde;
extern crate serde_json;
extern crate toml;

pub mod config;
pub mod json;
pub mod transform;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
