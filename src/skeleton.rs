use std::path::PathBuf;

use anyhow::anyhow;

use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};

use crate::file_access::FileAccess;

#[derive(Debug)]
pub struct Skeleton {
    pub syntax: SkeletonSyntax,
    pub css_var_general_prefix: String,
    pub css_var_syntax_prefix: String,
}

impl Skeleton {
    pub fn new(syntax: SkeletonSyntax) -> Self {
        Self {
            syntax,
            css_var_general_prefix: "--color-".to_string(),
            css_var_syntax_prefix: "--color-prettylights-syntax-".to_string(),
        }
    }

    pub fn to_css(
        &self,
        configuration_dir: &PathBuf,
    ) -> Result<String, anyhow::Error> {
        let syntax_map = self.syntax.to_json()?;
        let mut css = String::new();

        if let Value::Object(map) = syntax_map {
            self.process_map_to_css(&mut css, &map)?;
        }

        let theme_css = self.wrap_css_with_theme(configuration_dir, &css)?;

        Ok(theme_css)
    }

    fn process_map_to_css(
        &self,
        css: &mut String,
        map: &serde_json::Map<String, Value>,
    ) -> Result<(), anyhow::Error> {
        for (key, value) in map {
            match value {
                Value::String(v) => {
                    let css_var_name = if key == "color-scheme" {
                        key.replace("_", "-")
                    } else {
                        format!(
                            "{}{}",
                            self.css_var_general_prefix,
                            key.replace("_", "-")
                        )
                    };
                    css.push_str(&format!("{}: {};\n", css_var_name, v));
                }
                Value::Object(syntax_map) if key == "syntax" => {
                    self.process_syntax_map_to_css(css, syntax_map)?;
                }
                _ => return Err(anyhow!("Unexpected value type in JSON map.")),
            }
        }

        Ok(())
    }

    fn process_syntax_map_to_css(
        &self,
        css: &mut String,
        syntax_map: &serde_json::Map<String, Value>,
    ) -> Result<(), anyhow::Error> {
        for (syntax_key, syntax_value) in syntax_map {
            if let Value::String(syntax_color_value) = syntax_value {
                let css_var_name = format!(
                    "{}{}",
                    self.css_var_syntax_prefix,
                    syntax_key.replace("_", "-")
                );
                css.push_str(&format!(
                    "{}: {};\n",
                    css_var_name, syntax_color_value
                ));
            }
        }

        Ok(())
    }

    fn wrap_css_with_theme(
        &self,
        configuration_dir: &PathBuf,
        css: &str,
    ) -> Result<String, anyhow::Error> {
        let base_css = FileAccess::read_file(
            &configuration_dir.join("flavours").join("scaffolding.css"),
        )
        .unwrap();

        let theme_type = self.determine_theme_type(css)?;
        let css_with_class = format!(
            "@media (prefers-color-scheme: {}) {{\n  .markdown-body,\n  [data-theme=\"{}\"] {{\n{}\n  }}\n}}",
            theme_type, theme_type, css
        );

        let wrapped_css = format!("{}\n{}", css_with_class, base_css);
        Ok(wrapped_css)
    }

    fn determine_theme_type(&self, css: &str) -> Result<String, anyhow::Error> {
        let light_count = css.matches("light ").count();
        let dark_count = css.matches("dark").count();

        if light_count > dark_count {
            Ok("light".to_string())
        } else {
            Ok("dark".to_string())
        }
    }
}

impl SkeletonSyntax {
    pub fn to_yaml(&self) -> Result<String, serde_yaml::Error> {
        let yaml_string = serde_yaml::to_string(self)?;
        Ok(yaml_string)
    }

    fn to_json(&self) -> Result<Value, serde_json::Error> {
        to_value(&self)
    }

    pub fn from_yaml(yaml: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(yaml).map_err(|e| e.into())
    }

    pub fn from_json(json: Value) -> Result<Self, serde_json::Error> {
        from_value(json).map_err(Into::into)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SkeletonSyntax {
    #[serde(default)]
    pub syntax: Syntax,
    #[serde(rename = "color-scheme", default)]
    pub color_scheme: String,
    #[serde(rename = "fg-default", default)]
    pub fg_default: String,
    #[serde(rename = "fg-muted", default)]
    pub fg_muted: String,
    #[serde(rename = "fg-subtle", default)]
    pub fg_subtle: String,
    #[serde(rename = "canvas-default", default)]
    pub canvas_default: String,
    #[serde(rename = "canvas-subtle", default)]
    pub canvas_subtle: String,
    #[serde(rename = "border-default", default)]
    pub border_default: String,
    #[serde(rename = "border-muted", default)]
    pub border_muted: String,
    #[serde(rename = "neutral-muted", default)]
    pub neutral_muted: String,
    #[serde(rename = "accent-fg", default)]
    pub accent_fg: String,
    #[serde(rename = "accent-emphasis", default)]
    pub accent_emphasis: String,
    #[serde(rename = "success-fg", default)]
    pub success_fg: String,
    #[serde(rename = "success-emphasis", default)]
    pub success_emphasis: String,
    #[serde(rename = "attention-fg", default)]
    pub attention_fg: String,
    #[serde(rename = "attention-emphasis", default)]
    pub attention_emphasis: String,
    #[serde(rename = "attention-subtle", default)]
    pub attention_subtle: String,
    #[serde(rename = "danger-fg", default)]
    pub danger_fg: String,
    #[serde(rename = "danger-emphasis", default)]
    pub danger_emphasis: String,
    #[serde(rename = "done-fg", default)]
    pub done_fg: String,
    #[serde(rename = "done-emphasis", default)]
    pub done_emphasis: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Syntax {
    #[serde(rename = "comment", default)]
    pub comment: String,
    #[serde(rename = "constant", default)]
    pub constant: String,
    #[serde(rename = "entity", default)]
    pub entity: String,
    #[serde(rename = "storage-modifier-import", default)]
    pub storage_modifier_import: String,
    #[serde(rename = "entity-tag", default)]
    pub entity_tag: String,
    #[serde(rename = "keyword", default)]
    pub keyword: String,
    #[serde(rename = "string", default)]
    pub string: String,
    #[serde(rename = "variable", default)]
    pub variable: String,
    #[serde(rename = "brackethighlighter-unmatched", default)]
    pub bracket_highlighter_unmatched: String,
    #[serde(rename = "invalid-illegal-text", default)]
    pub invalid_illegal_text: String,
    #[serde(rename = "invalid-illegal-bg", default)]
    pub invalid_illegal_bg: String,
    #[serde(rename = "carriage-return-text", default)]
    pub carriage_return_text: String,
    #[serde(rename = "carriage-return-bg", default)]
    pub carriage_return_bg: String,
    #[serde(rename = "string-regexp", default)]
    pub string_regexp: String,
    #[serde(rename = "markup-list", default)]
    pub markup_list: String,
    #[serde(rename = "markup-heading", default)]
    pub markup_heading: String,
    #[serde(rename = "markup-italic", default)]
    pub markup_italic: String,
    #[serde(rename = "markup-bold", default)]
    pub markup_bold: String,
    #[serde(rename = "markup-deleted-text", default)]
    pub markup_deleted_text: String,
    #[serde(rename = "markup-deleted-bg", default)]
    pub markup_deleted_bg: String,
    #[serde(rename = "markup-inserted-text", default)]
    pub markup_inserted_text: String,
    #[serde(rename = "markup-inserted-bg", default)]
    pub markup_inserted_bg: String,
    #[serde(rename = "markup-changed-text", default)]
    pub markup_changed_text: String,
    #[serde(rename = "markup-changed-bg", default)]
    pub markup_changed_bg: String,
    #[serde(rename = "markup-ignored-text", default)]
    pub markup_ignored_text: String,
    #[serde(rename = "markup-ignored-bg", default)]
    pub markup_ignored_bg: String,
    #[serde(rename = "meta-diff-range", default)]
    pub meta_diff_range: String,
    #[serde(rename = "brackethighlighter-angle", default)]
    pub bracket_highlighter_angle: String,
    #[serde(rename = "sublimelinter-gutter-mark", default)]
    pub sublime_linter_gutter_mark: String,
    #[serde(rename = "constant-other-reference-link", default)]
    pub constant_other_reference_link: String,
}
