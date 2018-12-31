pub mod ts_enum {
    pub fn declaration(enum_name: String) -> String {
        format!("export const enum {}", enum_name)
    }

    pub fn opening_punctuation() -> String {
        String::from("{\n")
    }

    pub fn closing_punctuation() -> String {
        String::from("}\n")
    }
}
