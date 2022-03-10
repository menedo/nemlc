use super::content::Content;
use super::endpoint::NeiEndpoint;
use super::import::Import;
use crate::parse::parse;
use crate::parse::reader::BufReader;
use crate::utils::Strip;
use poirot::raster::ComCanvas;
use std::env;

#[derive(Debug, PartialEq)]
pub enum SegmentType {
    SegInit,
    SegImport,
    SegContent,
}

#[derive(Debug)]
pub struct Engine {
    pub seg_status: SegmentType,
    pub import_data: Import,
    pub content_data: Content,
}

#[derive(Debug)]
pub struct Config {
    pub name: String,
}

pub struct LineContext {
    pub line_number: usize,
    pub seg: Vec<String>,
}

impl Engine {
    pub fn init_config(line: String) -> Config {
        Config { name: line.clone() }
    }

    pub fn init() -> Self {
        Engine {
            seg_status: SegmentType::SegInit,
            import_data: Import::init(),
            content_data: Content::init(),
        }
    }

    pub fn compile(&mut self, config_data: Config) -> std::io::Result<NeiEndpoint> {
        let mut raw_data = Vec::new();
        let mut line_number: usize = 1;
        for line in BufReader::open(config_data.name)? {
            let datat = line?.to_string();
            let data = Strip::rstrip(datat, '\n');
            let line = parse::parse_simple(data);

            let context = LineContext {
                line_number: line_number,
                seg: line,
            };

            raw_data.push(context);

            line_number += 1;
        }

        for s in raw_data {
            self.parse_line(&s);
        }

        self.content_data.parse_level();
        self.content_data.parse_sub_arch();

        let arch = self.content_data.parse_build_arch();

        Ok(arch)
    }

    pub fn start(&mut self, config_data: Config) -> std::io::Result<()> {
        let mut raw_data = Vec::new();
        let mut line_number: usize = 1;
        for line in BufReader::open(config_data.name)? {
            let datat = line?.to_string();
            let data = Strip::rstrip(datat, '\n');
            let line = parse::parse_simple(data);

            let context = LineContext {
                line_number: line_number,
                seg: line,
            };

            raw_data.push(context);

            line_number += 1;
        }

        for s in raw_data {
            self.parse_line(&s);
        }

        self.content_data.parse_level();
        self.content_data.parse_sub_arch();

        self.generate_object_tree();

        Ok(())
    }

    pub fn generate_object_tree(&mut self) {
        let arch = self.content_data.parse_build_arch();
        let mut root = arch.create_object_tree();

        let arg = if env::args().count() == 2 {
            env::args().nth(1).unwrap()
        } else {
            panic!("Please enter a target file path")
        };

        let mut cc = ComCanvas::new(arg, (1600, 1600), None);
        let h = root.calc_box_height();

        let w0 = 100;
        let h0 = 100 + h / 2;
        root.draw_start(&mut cc, w0, h0);
    }

    pub fn seg_status_switch(&mut self, status: SegmentType) {
        if status == self.seg_status {
            return;
        }

        self.seg_status = status;
    }

    pub fn parse_line(&mut self, linec: &LineContext) {
        let line = &linec.seg;

        if line.len() < 1 {
            return;
        }

        let mut seg_skip = false;

        for s in line {
            match s.as_str() {
                "#[import]" => {
                    self.seg_status_switch(SegmentType::SegImport);
                    seg_skip = true;
                }
                "#[content]" => {
                    self.seg_status_switch(SegmentType::SegContent);
                    seg_skip = true;
                }
                _ => {}
            }
        }

        if !seg_skip {
            self.parse_raw_line(linec);
        }
    }

    pub fn parse_raw_line(&mut self, linec: &LineContext) {
        match self.seg_status {
            SegmentType::SegInit => {}
            SegmentType::SegImport => {
                self.import_data.parse_item(linec);
            }
            SegmentType::SegContent => {
                self.content_data.parse_item(linec);
            }
        }
    }
}
