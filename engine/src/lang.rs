pub enum Lang {
    Java,
    Cpp,
    Python,
}

impl Lang {
    /** 
     * Converts file extension to language
    */
    fn from_ext(s: &str) -> Option<Lang> {
        match s {
            "java" => Some(Lang::Java),
            "c" => Some(Lang::Cpp),
            "cc" => Some(Lang::Cpp),
            "cpp" => Some(Lang::Cpp),
            "py" => Some(Lang::Python),
            _ => None,
        }
    }
}