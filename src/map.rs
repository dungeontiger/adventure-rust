pub mod map {
    #[derive(Debug, serde::Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Map {
        name: String,
        description: String,
        start_location: String,
        end_location: String,
        locations: Vec<Location>
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Location {
        name: String,
        id: String,
        description: String,
        paths: Vec<Path>
    }

    #[derive(Debug, serde::Deserialize)]
    pub struct Path {
        name: String,
        id: String
    }

    impl Map {
        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_descritpion(&self) -> &str {
            &self.description
        }

        pub fn get_start_location(&self) -> &str {
            &self.start_location
        }

        pub fn get_end_location(&self) -> &str {
            &self.end_location
        }

        pub fn find_location(&self, id: &str) -> &Location {
            for location in &self.locations {
                if location.id == id {
                    return location;
                }
            }
            panic!("Did not find a location for the id.");
        }
    }

    impl Location {
        pub fn get_id(&self) -> &str {
            &self.id
        }

        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_descritpion(&self) -> &str {
            &self.description
        }

        pub fn get_paths(&self) -> &Vec<Path> {
            &self.paths
        }
    }

    impl Path {
        pub fn get_name(&self) -> &str {
            &self.name
        }

        pub fn get_id(&self) -> &str {
            &self.id
        }
    }
}
