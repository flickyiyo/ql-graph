pub struct Node {}

#[derive(Debug)]
pub struct Query<'a> {
    pub query_string: &'a String,
}

impl<'a> Query<'a> {
    pub fn new(query_string: &'a String) -> Self {
        Self { query_string }
    }

    pub fn from_query(&mut self) {
        
        let splt = self.query_string.split(" ");

        let vec: Vec<&str> = splt.collect();

        for item in vec {
            match item.to_lowercase().as_str() {
                "match" => {
                    
                },
                "create" => {
                    
                },
                x => {
                    if x != " " {

                    } 
                    if x == ";" {

                    }
                    if x.starts_with("(") {
                        let a = x.find(")");
                        match a {
                            Some(idx) => {
                                
                            },
                            None => {}
                        }
                    }
                }
            }
        }
    }

    pub fn get_node_properties(node_str: &str) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn detects_match() {
        let query = "MATCH (:Person {name: \"Dan\"}) - [:LOVES] -> (whom) RETURN whom;";


    }
}
