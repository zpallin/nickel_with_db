/*
 * nickel_with_db::helpers
 */

use std::collections::HashMap;
use mustache::builder::MapBuilder;
use mustache::Data;

/*
 *  InputFormatter
 */
pub struct InputFormatter;

impl InputFormatter {
    pub fn input_text<'a>(name: &str) -> String {
        format!(
            "<label for=\"{name}\">{name}</label> \
            <input id=\"{name}-input\" name=\"{name}\" type=\"text\">",
            name=name
        ).to_owned()
    }
    
    pub fn textarea(name: &str) -> String {
        format!(
            "<label for=\"{name}\">{name}</label> \
            <textarea id=\"{name}-input\" name=\"{name}\" type=\"text\" \
            rows=4 cols=10>",
            name=name
        ).to_owned()
    }
}

/*
 *  FormHelper
 */
#[derive(RustcEncodable)]
pub struct FormView {
    title: String,
    fields: Vec<String>,
}
pub struct FormHelper;

impl FormHelper {
    pub fn generic(title: &str, hash: &HashMap<&str, &str>) -> FormView {
        let mut fields = Vec::new();
        for (key, data_type) in hash {
            match data_type.as_ref() {
                "input_text" => {
                    fields.push(InputFormatter::input_text(key));
                },
                "textarea" => {
                    fields.push(InputFormatter::textarea(key));
                },
                _ => {
                    panic!("FormHelper::generic, asking for a non-existent
                           InputFormatter");
                }
            }
        }

        FormView {
            title: title.to_owned(),
            fields: fields,
        }
    }
}
