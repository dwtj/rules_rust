#[cfg(test)]
mod test {

    #[test]
    pub fn include_str_test() {
        let hello_world = format!(
            "{}, {}!",
            include_str!("source_data.txt").trim(),
            include_str!(concat!(env!("OUT_DIR"), "/../generated_data.txt")).trim()
        );
        println!("{}", hello_world);
        assert_eq!("Hello, world!", hello_world);
    }
}
