extern crate listpack;



#[cfg(test)]
mod tests {
    use ::listpack::raw::*;

    #[test]
    fn it_works() {
        let lp = new(ALLOCATOR);


        assert_eq!(2 + 2, 4);
    }
}