pub struct Finder;

impl Finder {
    pub fn next_point(s: &str, off: usize, target: char, is_equal: bool) -> Result<usize, String> {
        let ss = s.to_string();
        let bytes = &ss[off..];
        let bytes_new = bytes.to_string();
        let s_bytes = bytes_new.as_bytes();
        for (i, &item) in s_bytes.iter().enumerate() {
            let m = target as u8;
            if is_equal {
                if item == m {
                    return Ok(i);
                }
            } else {
                if item != m {
                    return Ok(i);
                }
            }
        }

        return Ok(s.len() - off);
    }

    pub fn next_sub(s: &str, off: usize, target: String) -> Option<usize> {
        let l = s[off..].to_string().find(&target);

        l
    }

    pub fn space_count(s: &str, target: char, is_equal: bool) -> usize {
        let ss = s.to_string();
        let s_bytes = ss.as_bytes();

        let mut count: usize = 0;
        for (_i, &item) in s_bytes.iter().enumerate() {
            let m = target as u8;
            if is_equal {
                if item == m {
                    count += 1;
                } else {
                    break;
                }
            } else {
                if item != m {
                    count += 1;
                } else {
                    break;
                }
            }
        }

        return count;
    }
}
