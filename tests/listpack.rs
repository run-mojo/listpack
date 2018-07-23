extern crate listpack;


#[cfg(test)]
mod tests {
    use ::listpack::*;

    #[test]
    fn it_works() {
        let lp = Listpack::new();

        assert_eq!(2 + 2, 4);
    }
}