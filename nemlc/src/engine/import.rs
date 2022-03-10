use super::engine::LineContext;
use crate::utils::{Concat, Finder, Strip};
use regex::Regex;

#[derive(Debug)]
pub struct Dependency {
    pub path: Option<String>,
    pub anchor: Option<String>,
    pub alias: Option<String>,
}

#[derive(Debug)]
pub struct Element {
    pub anchor: Option<String>,
    pub target: Option<String>,
}

#[derive(Debug)]
pub struct Import {
    pub data_list: Vec<Dependency>,
}

impl Import {
    pub fn init() -> Self {
        Import {
            data_list: Vec::new(),
        }
    }

    pub fn plain_alias_parse(&mut self, tc: Vec<&str>) -> Result<Dependency, std::io::Error> {
        let dst = tc[0];
        let alias = tc[1];

        let ele = self.plain_target_parse(dst).unwrap();
        let anc = ele.anchor;
        match anc {
            Some(anchor) => {
                let dep1 = Dependency {
                    path: Some(anchor),
                    anchor: ele.target,
                    alias: Some(alias.clone().to_string()),
                };

                return Ok(dep1);
            }
            None => {
                let dep1 = Dependency {
                    path: Some("/unknow".to_string()),
                    anchor: Some("/unknow".to_string()),
                    alias: Some("unknow".to_string()),
                };

                return Ok(dep1);
            }
        }
    }

    pub fn plain_target_parse(&mut self, _dest: &str) -> Result<Element, std::io::Error> {
        let dep = Element {
            anchor: Some("/unknow".to_string()),
            target: Some("unknow".to_string()),
        };

        return Ok(dep);
    }

    pub fn line_seg(s: &str) -> Result<Vec<String>, String> {
        let mut out_data = Vec::new();
        let mut index_now = 0;

        while index_now < s.len() {
            let mut next_delta = 0;

            let next = Strip::next_strip(s, index_now, ' ', true);
            let next_index = next.unwrap();
            next_delta += next_index;

            let ai = Finder::next_point(s, index_now + next_delta, '.', true);
            match ai {
                Ok(c) => {
                    next_delta = next_delta + c + 1;

                    let next = Strip::next_strip(s, index_now + next_delta, ' ', true);

                    let next_index = next.unwrap();
                    next_delta += next_index;

                    if s.as_bytes()[index_now + next_delta] == ('{' as u8) {
                        next_delta += 1;
                        let gap = Finder::next_point(s, index_now + next_delta, '}', true);
                        let gap_index = gap.unwrap();

                        next_delta = next_delta + gap_index + 1;

                        let next = Strip::next_strip(s, index_now + next_delta, ' ', true);

                        let next_index = next.unwrap();

                        next_delta += next_index;
                        if (index_now + next_delta) >= s.len() {
                            let raw_wait_str = &s[index_now..(next_delta + index_now)];
                            let wait_str = raw_wait_str.trim().to_string();
                            out_data.push(wait_str);
                        } else {
                            if s.as_bytes()[index_now + next_delta] != (',' as u8) {
                                return Err(String::from("Invalid"));
                            }

                            let raw_wait_str = &s[index_now..(next_delta + index_now)];
                            let wait_str = raw_wait_str.trim().to_string();
                            out_data.push(wait_str);

                            next_delta += 1;
                        }
                    } else {
                        let gap = Finder::next_point(s, index_now + next_delta, ',', true);

                        let gap_index = gap.unwrap();

                        next_delta = next_delta + gap_index + 1;

                        let raw_wait_str = &s[index_now..(next_delta + index_now - 1)];
                        let wait_str = raw_wait_str.trim().to_string();
                        out_data.push(wait_str);
                    }
                }
                Err(_) => {
                    println!("Invalid");
                    return Err(String::from("Invalid"));
                }
            }

            index_now += next_delta;
        }

        return Ok(out_data);
    }

    pub fn plain_mul_anchor_parse(
        &mut self,
        tc0: String,
    ) -> Result<Vec<Dependency>, std::io::Error> {
        let mut out_data = Vec::new();
        let tokens = Import::line_seg(tc0.as_str());
        match tokens {
            Ok(tc) => {
                for s in tc {
                    let mul_dep = self.plain_parse_target(s, "".to_string()).unwrap();
                    for d in mul_dep {
                        out_data.push(d);
                    }
                }
            }
            Err(_) => {}
        }

        Ok(out_data)
    }

    pub fn plain_mul_parse(&mut self, tc0: String) -> Result<Vec<Dependency>, std::io::Error> {
        let mut out_data = Vec::new();
        let tokens = tc0.split(",");
        let tc: Vec<&str> = tokens.collect();

        for s in tc {
            let line = s.trim().to_string();

            let dep = self.plain_atom_parse(line).unwrap();

            out_data.push(dep);
        }

        Ok(out_data)
    }

    pub fn plain_atom_parse(&mut self, tc0: String) -> Result<Dependency, std::io::Error> {
        let tokens = tc0.split(" as ");
        let tc: Vec<&str> = tokens.collect();

        let plain_count = tc.len();
        if plain_count == 1 {
            let anchor = tc[0].trim().to_string();
            let anchor_new = Strip::colon_strip(anchor);
            let anchor_real = anchor_new.trim().to_string();
            if anchor_real.len() < 1 {
                let dep = Dependency {
                    path: Some("/unknow".to_string()),
                    anchor: Some("/unknow".to_string()),
                    alias: Some("unknow".to_string()),
                };

                return Ok(dep);
            }

            let ele = Dependency {
                path: None,
                anchor: Some(anchor_real),
                alias: None,
            };

            return Ok(ele);
        } else if plain_count == 2 {
            let target = tc[0].trim().to_string();
            let alias = tc[1].trim().to_string();

            let new_target = Strip::colon_strip(target);
            let new_alias = Strip::colon_strip(alias);

            let real_target = new_target.trim().to_string();
            let real_alias = new_alias.trim().to_string();

            let dep = Dependency {
                path: None,
                anchor: Some(real_target),
                alias: Some(real_alias),
            };

            return Ok(dep);
        } else {
            //Error
            let dep = Dependency {
                path: Some("/unknow".to_string()),
                anchor: Some("/unknow".to_string()),
                alias: Some("unknow".to_string()),
            };

            return Ok(dep);
        }
    }

    pub fn plain_parse(&mut self, tc0: Vec<&str>) -> Result<Vec<Dependency>, std::io::Error> {
        let tot_len = tc0.len();
        let prefix = Concat::slash_concat(&tc0, tot_len - 1, "".to_string());

        let obj = tc0[tot_len - 1];
        let raw_obj = obj.to_string();

        return self.plain_parse_impl(raw_obj, prefix);
    }

    pub fn plain_parse_target(
        &mut self,
        raw_obj: String,
        prefix: String,
    ) -> Result<Vec<Dependency>, std::io::Error> {
        let tokens = raw_obj.split(".");
        let tc: Vec<&str> = tokens.collect();
        let tc_len = tc.len();
        if tc_len < 2 {
            let dep1 = Dependency {
                path: Some("/unknow".to_string()),
                anchor: Some("/unknow".to_string()),
                alias: Some("unknow".to_string()),
            };

            let mut out_data = Vec::new();
            out_data.push(dep1);

            return Ok(out_data);
        }

        let anchor = tc[0].to_string();
        let anc_len = anchor.len();
        let tar_off = anc_len + 1;

        let tar = &raw_obj[tar_off..];

        if tar.starts_with("{") && raw_obj.ends_with("}") {
            let tar_len = tar.len();
            if tar_len < 3 {
                let dep = Dependency {
                    path: Some("/unknow".to_string()),
                    anchor: Some("/unknow".to_string()),
                    alias: Some("unknow".to_string()),
                };

                let mut out_data = Vec::new();
                out_data.push(dep);

                return Ok(out_data);
            }

            let mul_tar_obj = tar[1..(tar_len - 1)].to_string();
            let path = Concat::raw_concat(prefix.clone(), "/".to_string(), anchor);
            let mut mul_dep = self.plain_mul_parse(mul_tar_obj).unwrap();
            for ref mut d in &mut mul_dep {
                d.path = Some(path.clone());
            }

            return Ok(mul_dep);
        } else {
            let mut dep = self.plain_atom_parse(tar.to_string()).unwrap();
            let path = Concat::raw_concat(prefix.clone(), "/".to_string(), anchor);

            dep.path = Some(path);

            let mut out_data = Vec::new();
            out_data.push(dep);

            return Ok(out_data);
        }
    }

    pub fn plain_parse_impl(
        &mut self,
        raw_obj: String,
        prefix: String,
    ) -> Result<Vec<Dependency>, std::io::Error> {
        if raw_obj.starts_with("{") && raw_obj.ends_with("}") {
            let mul_obj = raw_obj.len();
            if mul_obj < 3 {
                let dep = Dependency {
                    path: Some("/unknow".to_string()),
                    anchor: Some("/unknow".to_string()),
                    alias: Some("unknow".to_string()),
                };

                let mut out_data = Vec::new();
                out_data.push(dep);

                return Ok(out_data);
            }

            let mul_raw_obj = raw_obj[1..(mul_obj - 1)].to_string();

            let mut mul_a_dep = self.plain_mul_anchor_parse(mul_raw_obj).unwrap();

            for ref mut d in &mut mul_a_dep {
                match &d.path {
                    Some(p) => {
                        let path =
                            Concat::raw_concat(prefix.clone(), "/".to_string(), p.to_string());
                        d.path = Some(path.clone());
                    }
                    None => {}
                }
            }

            return Ok(mul_a_dep);
        }

        return self.plain_parse_target(raw_obj, prefix);
    }

    pub fn alis_simple_process(&mut self, s: &String) -> Result<Vec<Dependency>, std::io::Error> {
        let tokens = s.split("/");
        let tc: Vec<&str> = tokens.collect();
        let ptc = self.plain_parse(tc);
        return ptc;
    }

    pub fn extract(&mut self, s: &String) -> Result<Vec<Dependency>, std::io::Error> {
        let out_data = Vec::new();
        let r1 = Regex::new(r"(.+/)?(.*)").unwrap();
        if r1.is_match(&s) {
            let ptc = self.alis_simple_process(s);
            println!("import = {:#?}", ptc);

            return ptc;
        }

        Ok(out_data)
    }

    pub fn parse_item(&mut self, linec: &LineContext) {
        let line = &linec.seg;
        for rst in line {
            let rs = rst.trim().to_string();

            if rs.len() == 0 {
                continue;
            }
            let new_s = rs.clone();
            if new_s.starts_with("//") {
                continue;
            }

            if !new_s.starts_with("use ") {
                println!("Error line: {} {:?}", linec.line_number, new_s)
            }

            let s = new_s[4..].to_string();
            let _r = self.extract(&s);
        }
    }
}
