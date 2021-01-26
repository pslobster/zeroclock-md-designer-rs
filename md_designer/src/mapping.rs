use std::collections::HashMap;

use anyhow::Result;
use pulldown_cmark::Tag;

use crate::{constant::AUTO_INCREMENT_KEY, rule::Rule, utils::cmarktag_stringify};

#[derive(Debug, PartialEq)]
pub struct Mapping {
    pub mappings: Vec<HashMap<String, usize>>,
}

impl Mapping {
    pub fn new(rule: &Rule) -> Result<Self> {
        let mut mappings = vec![];
        rule.doc.blocks.iter().for_each(|block| {
            let mut data = HashMap::new();
            block.columns.iter().enumerate().for_each(|(idx, column)| {
                if column.auto_increment {
                    data.insert(AUTO_INCREMENT_KEY.clone(), idx);
                } else {
                    data.insert(column.cmark_tag.clone(), idx);
                }
            });
            mappings.push(data);
        });
        Ok(Mapping { mappings })
    }
}

impl Mapping {
    pub fn get_idx(&self, block_idx: usize, tag: &Tag<'_>) -> Option<&usize> {
        if let Some(map) = self.mappings.get(block_idx) {
            if let Some(tag_str) = cmarktag_stringify(tag) {
                return map.get(&tag_str);
            }
        }
        None
    }

    pub fn get_auto_increment_idx(&self, block_idx: usize) -> Option<&usize> {
        if let Some(map) = self.mappings.get(block_idx) {
            return map.get(&AUTO_INCREMENT_KEY.clone());
        }
        None
    }

    pub fn get_size(&self, block_idx: usize) -> Option<usize> {
        if let Some(map) = self.mappings.get(block_idx) {
            return Some(map.len());
        }
        None
    }
}

impl Default for Mapping {
    fn default() -> Self {
        Self { mappings: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::constant::AUTO_INCREMENT_KEY;

    #[test]
    fn test_mapping() {
        let rule = Rule::marshal(
            r#"
doc:
  blocks:
    - block:
      - column: No
        isNum: true
      - group: Variation
        columns:
        - column: Variation 1
          md: Heading2
        - column: Variation 2
          md: Heading3
        - column: Variation 3
          md: Heading4
        - column: Variation 4
          md: Heading5
        - column: Variation 5
          md: Heading6
        - column: Variation 6
          md: Heading7
        - column: Variation 7
          md: Heading8
      - column: Description
        md: List
            "#,
        )
        .unwrap();
        let mapping = Mapping::new(&rule).unwrap();
        let mut map = HashMap::new();
        map.insert(AUTO_INCREMENT_KEY.clone(), 0);
        map.insert("Heading2".to_string(), 1);
        map.insert("Heading3".to_string(), 2);
        map.insert("Heading4".to_string(), 3);
        map.insert("Heading5".to_string(), 4);
        map.insert("Heading6".to_string(), 5);
        map.insert("Heading7".to_string(), 6);
        map.insert("Heading8".to_string(), 7);
        map.insert("List".to_string(), 8);
        let expected = Mapping {
            mappings: vec![map],
        };
        assert_eq!(expected, mapping);
    }
}
