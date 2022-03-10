pub struct Concat;

impl Concat {
    pub fn raw_concat(prefix: String, e: String, anchor: String) -> String {
        if prefix.len() > 0 && anchor.len() > 0 {
            let end = format!("{}{}{}", prefix, e, anchor);
            return end;
        }

        let end = format!("{}{}", prefix, anchor);

        end
    }

    pub fn slash_concat(tc0: &Vec<&str>, count: usize, anchor: String) -> String {
        let mut start = "".to_string();

        if tc0.len() >= count && count > 0 {
            start = tc0[0].to_string();
        }

        let mut i = 1;
        while i < count {
            let ele = tc0[i].to_string();
            if start.len() > 0 && ele.len() > 0 {
                start = format!("{}/{}", start, ele);
            } else {
                start = format!("{}{}", start, ele);
            }

            i += 1;
        }

        if start.len() > 0 && anchor.len() > 0 {
            start = format!("{}/{}", start, anchor);
        } else {
            start = format!("{}{}", start, anchor);
        }

        start
    }
}
