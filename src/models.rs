use similar::{utils, Algorithm, ChangeTag};

#[derive(Default, Clone)]
pub struct Model {
    input_source: String,
    input_target: String,
    source_text: String,
    source_style: String,
    target_text: String,
    target_style: String,
}

impl Model {
    pub fn source_text(&self) -> &String {
        &self.source_text
    }
    pub fn source_style(&self) -> &String {
        &self.source_style
    }
    pub fn target_text(&self) -> &String {
        &self.target_text
    }
    pub fn target_style(&self) -> &String {
        &self.target_style
    }
    pub fn set_source(&mut self, value: String) {
        self.input_source = value;
        self.diff();
    }
    pub fn set_target(&mut self, value: String) {
        self.input_target = value;
        self.diff();
    }
    fn diff(&mut self) {
        for (tag, text) in
            utils::diff_chars(Algorithm::Myers, &self.input_source, &self.input_target)
        {
            match tag {
                ChangeTag::Delete => {
                    self.source_text.push_str(text);
                    self.source_style.push_str(&"C".repeat(text.len()));
                    self.target_text.push_str(&" ".repeat(text.len()));
                    self.target_style.push_str(&" ".repeat(text.len()));
                }
                ChangeTag::Insert => {
                    self.source_text.push_str(&" ".repeat(text.len()));
                    self.source_style.push_str(&" ".repeat(text.len()));
                    self.target_text.push_str(text);
                    self.target_style.push_str(&"B".repeat(text.len()));
                }
                ChangeTag::Equal => {
                    self.source_text.push_str(text);
                    self.source_style.push_str(&"A".repeat(text.len()));
                    self.target_text.push_str(text);
                    self.target_style.push_str(&"A".repeat(text.len()));
                }
            }
        }
    }
}
