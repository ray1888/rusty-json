#[derive(Debug)]
pub enum LeptType {
    LeptNull,
    LeptFalse,
    LeptTrue,
    LeptNumber,
    LeptString,
    LeptArray,
    LeptObject,
}

#[derive(Debug)]
pub enum LeptResult {
    LeptParseOk,
    LeptParseExpectValue,
    LeptParseInvalidValue,
    LeptParseRootNotSingular,
}

pub struct LeptValue {
    t:LeptType,
}

pub struct LeptJson{}


impl LeptJson{
    fn lept_parse(v: LeptValue, json: String) -> i32 {
        return 1;
    }
 
 
    fn lept_get_type(v:  LeptValue) -> LeptType {
        return v.t;
    }
}


