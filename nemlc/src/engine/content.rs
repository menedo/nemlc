use super::engine::LineContext;
use crate::engine::NeiEndpoint;
use crate::utils::Finder;
use regex::Regex;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Item {
    pub level: usize,
    pub name: Option<String>,
    pub name_macro: Option<String>,
    pub dir: Option<String>,
    pub relation: Option<String>,
    pub rel_macro: Option<String>,

    pub index: usize,
    pub root: usize,

    pub sub_list: Vec<usize>,
}

#[derive(Debug)]
pub struct Entity {
    pub index: usize,
    pub body: Option<String>,
    pub attr: Option<String>,
    pub level: usize,
    pub root: usize,
}

#[derive(Debug, PartialEq)]
pub enum ItemStatus {
    ItemWaitRoot,
    ItemWaitLeaf,
}

#[derive(Debug)]
pub struct Content {
    pub current_level: usize,
    pub data_list: Rc<RefCell<Vec<Item>>>,
}

pub struct LevelTable {
    pub table: Vec<usize>,
    pub current_level: usize,
}

impl Content {
    pub fn init() -> Self {
        let c = Content {
            current_level: 0,
            data_list: Rc::new(RefCell::new(Vec::new())),
        };

        let item = Item {
            name: None,
            name_macro: None,
            dir: None,
            relation: None,
            rel_macro: None,
            level: 0,
            index: 0,
            root: 0,
            sub_list: Vec::new(),
        };

        c.data_list.borrow_mut().push(item);

        c
    }

    /*
     *  |index| self|root |  0  |  1  |  2  |  3  |
     *  |  0  |  -  |  -  |  0  |  -  |  -  |  -  |
     *  |  1  |  0  |  0  |  0  |  1  |  -  |  -  |
     *  |  2  |  1  |  1  |  0  |  1  |  2  |  -  |
     *  |  3  |  2  |  2  |  0  |  1  |  2  |  2  |
     *  |  4  |  1  |  1  |  0  |  1  |  4  |  -  |
     *  |  5  |  2  |  4  |  0  |  1  |  4  |  5  |
     */
    pub fn parse_item_level(&mut self, index: usize, table: &mut LevelTable) -> usize {
        let data_table = &mut self.data_list.borrow_mut();
        let ent = &mut data_table[index];

        if ent.level > table.current_level {
            return 0;
        }

        if table.table.len() != (table.current_level + 1) {
            // TODO: Error
        }

        let root_index = table.table[ent.level];

        if (ent.level + 1) > table.current_level {
            table.table.push(ent.index);
        } else {
            table.table[ent.level + 1] = ent.index;
            table.table.truncate(ent.level + 2);
        }

        table.current_level = ent.level + 1;

        return root_index;
    }

    pub fn parse_root_item(&mut self, s: &String) -> Result<Item, String> {
        let tokens = s.split(" ");
        let tc: Vec<&str> = tokens.collect();
        let tc_len = tc.len();
        if tc_len > 2 {
            return Err(String::from("Invalid"));
        }

        let name = tc[0].trim().to_string();
        if tc_len == 1 {
            let item = Item {
                level: 0,
                name: Some(name),
                name_macro: None,
                dir: None,
                relation: None,
                rel_macro: None,
                index: 0,
                root: 0,
                sub_list: Vec::new(),
            };

            return Ok(item);
        } else {
            let name_macro = tc[1].trim().to_string();
            let item = Item {
                level: 0,
                name: Some(name),
                name_macro: Some(name_macro),
                dir: None,
                relation: None,
                rel_macro: None,
                index: 0,
                root: 0,
                sub_list: Vec::new(),
            };

            return Ok(item);
        }
    }

    pub fn element_parse(&mut self, s: &String) -> Option<Entity> {
        let new_s = s.trim().to_string();

        let ai = Finder::next_point(&new_s, 0, ' ', true);
        let ai_index = ai.unwrap();

        let body = &new_s[..ai_index].trim().to_string();
        let mut attr: String = "".to_string();

        if body.len() < new_s.len() {
            let attr_t = &new_s[body.len()..].trim().to_string();
            attr = attr_t.trim().to_string();
        }

        let mut a = Entity {
            body: None,
            attr: None,
            index: 0,
            root: 0,
            level: 0,
        };

        if body.len() > 0 {
            a.body = Some(body.to_string());
        }

        if attr.len() > 0 {
            a.attr = Some(attr);
        }

        Some(a)
    }

    pub fn parse_plain(&mut self, s: &String) -> Result<Vec<Item>, std::io::Error> {
        let mut out_data = Vec::new();

        if !s.starts_with("  ") {
            let root = self.parse_root_item(s).unwrap();

            self.current_level = 0;

            out_data.push(root);
        } else {
            let mut level = Finder::space_count(s.as_str(), ' ', true);

            if (level % 2) != 0 {
                return Ok(out_data);
            }

            level = level / 2;

            self.current_level = level;

            let r = Regex::new(r"(.*)[\+\-\*](.*)").unwrap();
            if !r.is_match(s) {
                let raw_name = s.trim().to_string();
                let name_entity = self.element_parse(&raw_name).unwrap();

                let item = Item {
                    level: level,
                    name: name_entity.body,
                    name_macro: name_entity.attr,
                    dir: None,
                    relation: None,
                    rel_macro: None,
                    index: 0,
                    root: 0,
                    sub_list: Vec::new(),
                };

                out_data.push(item);
            } else {
                let mut count: usize = 0;
                let mut tmp_data = Vec::new();
                for (_i, c) in r.captures_iter(&s).enumerate() {
                    for j in 1..c.len() {
                        count += 1;
                        tmp_data.push(c[j].trim().to_string());
                    }
                }

                if count == 2 {
                    let raw_name = tmp_data[0].trim().to_string();
                    let raw_rel = tmp_data[1].trim().to_string();

                    if raw_name.len() == 0 || raw_rel.len() == 0 {
                        return Ok(out_data);
                    }

                    let r = Regex::new(r"[ ][\+\-\*][ ]").unwrap();
                    if !r.is_match(s) {
                        return Ok(out_data);
                    }

                    let mut dir: String = "".to_string();

                    for (_i, c) in r.captures_iter(&s).enumerate() {
                        for j in 0..c.len() {
                            dir = c[j].trim().to_string();
                            break;
                        }
                    }
                    let name_entity = self.element_parse(&raw_name).unwrap();
                    let attr_entity = self.element_parse(&raw_rel).unwrap();

                    let item = Item {
                        level: level,
                        name: name_entity.body,
                        name_macro: name_entity.attr,
                        dir: Some(dir),
                        relation: attr_entity.body,
                        rel_macro: attr_entity.attr,
                        index: 0,
                        root: 0,
                        sub_list: Vec::new(),
                    };

                    out_data.push(item);
                } else {
                    println!("error count Error");
                }
            }
        }

        Ok(out_data)
    }

    pub fn extract(&mut self, s: &String) -> Result<Vec<Item>, std::io::Error> {
        let ptc = self.parse_plain(s);
        ptc
    }

    pub fn update_sub_list(&mut self, root_index: usize, sub_index: usize) {
        let item = &mut self.data_list.borrow_mut()[root_index];
        item.sub_list.push(sub_index);
    }

    pub fn get_root_from_index(&self, index: usize) -> usize {
        let item = &self.data_list.borrow()[index];
        let root = &self.data_list.borrow()[item.root];
        return root.index;
    }

    pub fn parse_sub_arch(&mut self) {
        let mut index = 1;
        while index < self.data_list.borrow().len() {
            let root_index = self.get_root_from_index(index);
            self.update_sub_list(root_index, index);
            index += 1;
        }
    }

    pub fn build_sub_list(
        &self,
        index: usize,
        name: Option<String>,
        processed: &mut [usize],
    ) -> NeiEndpoint {
        let mut end = NeiEndpoint::new(index, name);
        let item = &self.data_list.borrow()[index];

        for i in &item.sub_list {
            if processed[*i] == 1 {
                continue;
            }
            let item_sub = &self.data_list.borrow()[*i];
            let name = item_sub.name.clone();
            let sub_end = self.build_sub_list(*i, name, processed);
            end.attach(sub_end);

            processed[*i] = 1;
        }

        end
    }

    pub fn parse_build_arch(&mut self) -> NeiEndpoint {
        let mut end = NeiEndpoint::new(0, Some("root".to_string()));
        let mut processed = Vec::new();
        let mut index = 0;
        while index < self.data_list.borrow().len() {
            processed.push(0);
            index += 1;
        }

        index = 1;
        while index < self.data_list.borrow().len() {
            if processed[index] == 0 {
                let name = self.data_list.borrow()[index].name.clone();
                let sub = self.build_sub_list(index, name, &mut processed);
                end.attach(sub);
            }
            index += 1;
        }

        end
    }

    pub fn parse_level(&mut self) {
        let mut table = LevelTable {
            table: Vec::new(),
            current_level: 0,
        };

        table.table.push(0);

        let mut index = 1;
        while index < self.data_list.borrow().len() {
            self.data_list.borrow_mut()[index].index = index;
            let root_index = self.parse_item_level(index, &mut table);
            self.data_list.borrow_mut()[index].root = root_index;

            index += 1;
        }
    }

    pub fn empty_item(&mut self) -> Item {
        Item {
            name: None,
            name_macro: None,
            dir: None,
            relation: None,
            rel_macro: None,
            level: 0,
            index: 0,
            root: 0,
            sub_list: Vec::new(),
        }
    }

    pub fn parse_item(&mut self, linec: &LineContext) {
        let line = &linec.seg;
        for rs in line {
            if rs.len() == 0 {
                continue;
            }
            let new_s = rs.clone();
            if new_s.starts_with("//") {
                continue;
            }

            let ptc = self.extract(&new_s).unwrap();
            for item in ptc {
                self.data_list.borrow_mut().push(item);
            }
        }
    }
}
