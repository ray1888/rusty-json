pub mod leptjson;

#[cfg(test)]
mod tests {

    
    #[test]
    fn parse_null() {
        let mut v = LeptValue{
            t: LeptResult::LeptNull,
        };
        let l = LeptJson{};
        assert_eq!(LeptResult::LeptParseOk, l::lept_parse(&v, "null"));
        assert_eq!(LeptResult::LeptNull, l::lept_get_type(&v));
    }


    // #[test]
    // fn parse_expect_value() {
    //     assert_eq!(2 + 2, 4);
    // }

    // #[test]
    // fn parse_invalid_value() {
    //     assert_eq!(2 + 2, 4);
    // }

    // #[test]
    // fn parse_root_not_singular() {
    //     assert_eq!(2 + 2, 4);
    // }

}
