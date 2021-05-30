use std::collections::HashMap;

pub fn render(fmt : &String, replacements : &HashMap<String, String>) -> String {
    let mut res = (*fmt).clone();

    for (k,v) in replacements {
        let to_rep = format!("{{{}}}", v);
        let replacement = format!("{:12}", k);
        res = res.replace(&to_rep, &replacement);
    }
    
    res
}
