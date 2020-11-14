enum LeptType {
    LeptNull
    LeptFalse
    LeptTrue 
    LeptNumber
    LeptString
    LeptArray
    LeptObject
}

enum LeptResult {
    LeptParseOk 
    LeptParseExpectValue
    LeptParseInvalidValue
    LeptParseRootNotSingular
}

type lept_value {
    type:LeptType
}



