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
                            None => {
                                
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn get_node_properties(node_str: &str) {

    }
}

pub fn divide_tokens(query: &String) -> Option<Vec<&str>> {
    let vec_str = query.as_str().split("");
    let chars = query.chars();

    let mut inside_string = false;
    let mut after_bar = false;
    let mut after_dash = false;
    let mut inside_node = false;
    let mut inside_edge = false;
    let mut inside_props = false;
    let mut tokens: Vec<String> = vec![];

    let mut owned_string = "".to_owned();

    for c in chars {
        if inside_string && c != '"' {
            owned_string.push(c);
            continue;
        }
        match c {
            '(' => {},
            ')' => {},
            _ => {}
        }
        
        if c == '\\' {
            if !after_bar {
                after_bar = true;
            } else {
                owned_string.push(c);
                after_bar = false;
            }
            continue;
        }
        if c == ' ' {
            if inside_string {
                owned_string.push(c);
            } else {
                tokens.push(owned_string.clone());
                owned_string = "".to_string();
            }
            continue;
        }
        if c == '"' {
            if !inside_string {
                owned_string.push(c);
                inside_string = true;
            } else {
                owned_string.push(c);
            }
            continue;
        }
        if c == '(' {

        }
        if c == ')' {

        }
        if c == '-' {

        }
        if c == '>' {

        }
        if c == '[' {

        }
        if c == ']' {

        }
        if c == '\'' {
            continue;
        }
        if c == '\n' {
            continue;
        }
        if c.is_numeric() {
            owned_string.push(c);
            continue;
        }
        if c.is_alphabetic() {
            owned_string.push(c);
            continue;
        }
        if c.is_control() {

        }
    }



    None
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
