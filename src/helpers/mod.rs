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
    pub fn form_wrap(name: &str, inner: &str) -> String {
        format!(
            "<form id=\"{name}\" method=\"post\" \
            action=\"/{name}/submit\"><br/> \
            {inner} <br/> \
            </form>",
            name=name,
            inner=inner
        ).to_owned()
    }

    pub fn input_text(name: &str) -> String {
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
            rows=4 cols=30></textarea>",
            name=name
        ).to_owned()
    }
    pub fn submit() -> String {
        format!(
            "<input type=\"submit\" value=\"Post\" />"
        ).to_owned()
    }

    fn concat_fields(fields: &String, field: &String) -> String {
        format!("{}{}<br/>",fields, field)
    }
}

/*
 *  FormHelper
 */
#[derive(RustcEncodable)]
pub struct FormView {
    title: String,
    form: String,
}
pub struct FormHelper;

impl FormHelper {

    pub fn generic(title: &str, schema: &Vec<(&str,&str)>) -> FormView {

        let mut fields = "".to_owned();

        for i in 0..schema.len() {
            let mut field = "".to_owned();
            let key = schema[i].0;
            let data_type = schema[i].1;
            match data_type {
                "input_text" => {
                    field = InputFormatter::input_text(key);
                },
                "textarea" => {
                    field = InputFormatter::textarea(key);
                },
                _ => {
                    panic!("FormHelper::generic, asking for a non-existent
                           InputFormatter");
                }
            }
            fields = InputFormatter::concat_fields(&fields, &field);
        }

        fields = InputFormatter::concat_fields(&fields, &InputFormatter::submit());

        FormView {
            title: title.to_owned(),
            form: InputFormatter::form_wrap(&title, &fields),
        }
    }
}
