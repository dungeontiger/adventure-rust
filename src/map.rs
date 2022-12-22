pub mod map {
    #[derive(Debug, serde::Deserialize)]
    pub struct Map {
        name: String,
        description: String
    }

    impl Map {
        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_descritpion(&self) -> &str {
            &self.description
        }
    }
}
