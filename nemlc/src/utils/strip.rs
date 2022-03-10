pub struct Strip;

impl Strip {
    pub fn rstrip(input: String, label: char) -> String {
        let mut tot_len = input.len();
        while tot_len > 0 {
            if input.as_bytes()[tot_len - 1] == (label as u8) {
                tot_len -= 1;
            } else {
                break;
            }
        }

        return input[0..tot_len].to_string();
    }

    pub fn colon_strip(input: String) -> String {
        if input.starts_with("\"") && input.ends_with("\"") {
            if input.len() < 3 {
                return "".to_string();
            }

            let r = &input[1..(input.len() - 2)];
            r.to_string()
        } else {
            input
        }
    }

    pub fn next_strip(s: &str, off: usize, target: char, is_equal: bool) -> Result<usize, String> {
        let ss = s.to_string();
        let bytes = &ss[off..];
        let bytes_new = bytes.to_string();
        let s_bytes = bytes_new.as_bytes();
        for (i, &item) in s_bytes.iter().enumerate() {
            let m = target as u8;
            if is_equal {
                if item != m {
                    return Ok(i);
                }
            } else {
                if item == m {
                    return Ok(i);
                }
            }
        }

        return Ok(s.len() - off);
    }
}
