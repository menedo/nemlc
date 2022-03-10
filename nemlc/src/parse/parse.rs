pub fn parse_simple(line: String) -> Vec<String> {
    let new_line = line.clone();

    let mut out_data = Vec::new();
    out_data.push(new_line);

    out_data
}

pub fn parse_simple_vector(line: String) -> Vec<String> {
    let new_line = line.clone();
    let split = new_line.split_whitespace();
    let vec: Vec<&str> = split.collect();

    let mut out_data = Vec::new();
    for s in vec {
        let token = s.clone().to_string();
        out_data.push(token);
    }

    out_data
}
