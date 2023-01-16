/*use serde::{Serialize, Deserialize};
use serde_yaml;
use serde_derive::{Serialize, Deserialize};
use crate::color::*;



/*pub struct Art {
    name: String,
    art: Vec<String>,
}*/
#[derive(Debug, Serialize, Deserialize)]
pub struct Art {
    name: String,
    colors: Vec<(usize, usize, String)>,
    art: Vec<String>,
}
*/
/*pub fn main() {
    let art = Art {
        colors: vec![(10,10, RED.to_string())],
        name: "my_art".to_string(),
        art: vec![
            "    ____        __        __".to_string(),
            "   / __ \\____ _/ /_____ _/ /_".to_string(),
            "  / /_/ / __ `/ __/ __ `/ __/".to_string(),
            " / _, _/ /_/ / /_/ /_/ / /_".to_string(),
            "/_/ |_|\\__,_/\\__/\\__,_/\\__/".to_string(),
        ],
    };



    //let art: Art = serde_yaml::from_str(&yaml_str).unwrap();

    let mut colored_art = String::new();
    for (i, line) in art.art.iter().enumerate() {for (start, end, color) in art.colors.iter().filter(|(start, end, _)| i >= *start && i <= *end) {
        colored_art.push_str(color);
        colored_art.push_str(&line[*start..*end]);
        colored_art.push_str(RESET);
    }

        colored_art.push_str("\n");



    }



    let yaml_str = serde_yaml::to_string(&art).unwrap();

// write to file
    std::fs::write("art.yaml", &yaml_str).unwrap();

// read from file


    let yaml_str = std::fs::read_to_string("art.yaml").unwrap();
    let art: Art = serde_yaml::from_str(&yaml_str).unwrap();
}
*/