extern crate combine;
use self::combine::*;
use self::combine::combinator::{Many, SepBy};
use self::combine::primitives::{Consumed, Stream};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    IntObject(i32),
    Boolean(bool),
    String(String),
    VecObject(Vec<Object>),
    StructVecObject(Vec<HashMap<String, Object>>),
    RandomText(String),
}
pub type Section = HashMap<String, Object>;
pub type Sections = Vec<Section>;

fn title_parser(input: State<&str>) -> ParseResult<String, &str> {
    between(token('['), token(']'), many1(alpha_num())).parse_state(input)
}

fn string_parser(input: State<&str>) -> ParseResult<String, &str> {
    fn escaped_char_parser(input: State<&str>) -> ParseResult<char, &str> {
        let (c, input) = try!(any().parse_lazy(input));
        let mut back_slash_char = satisfy(|c| "\"\\/bfnrt".chars().find(|x| *x == c).is_some()).map(|c| {
            match c {
                '"' => '"',
                '\\' => '\\',
                '/' => '/',
                'b' => '\u{0008}',
                'f' => '\u{000c}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                c => c//Should never happen
            }
        });
        match c {
            '\\' => input.combine(|input| back_slash_char.parse_state(input)),
            '"'  => Err(Consumed::Empty(ParseError::from_errors(input.into_inner().position, Vec::new()))),
            _    => Ok((c, input))
        }
    }
    optional(string("_("))
        .with(between(char('"'),
                      char('"'),
                      many(parser(escaped_char_parser))
                      ))
        .skip(optional(char(')'))).parse_state(input)
}

fn boolean_parser(input : State<&str>) -> ParseResult<Object, &str> {
    unimplemented!()
}

fn wierd_exception(input : State<&str>) -> ParseResult<Object, &str> {
    unimplemented!()
}

fn single_object_parser(input : State<&str>) -> ParseResult<Object, &str> {
    let integer_parser = spaces().with(many1(digit())).map(|string : String| Object::IntObject(string.parse::<i32>().unwrap()));
    let string_object_parser = parser(string_parser).map(|string| Object::String(string));
    integer_parser.or(parser(boolean_parser)).or(string_object_parser).or(parser(wierd_exception)).parse_state(input)
}

fn struct_parser(input: State<&str>) -> ParseResult<(Vec<String>, Vec<Vec<Object>>), &str> {
    let comma_parser = spaces().with(char(',')).skip(spaces());
    let title_parser = char('{').with(spaces()).with(sep_by(parser(string_parser), comma_parser.clone()));
    let row_parser = many(spaces().with(sep_by(parser(single_object_parser), comma_parser)));
    // fn create_map(tuple : (vec<String>, vec<vec<Object>>));
    title_parser.and(row_parser).parse_state(input)
}

fn object_parser(input : State<&str>) -> ParseResult<Object, &str> {
    unimplemented!()
}

fn assignment_parser(input : State<&str>) -> ParseResult<(String, Object), &str> {
    unimplemented!()
}

fn section_parser(input : State<&str>) -> ParseResult<(String, HashMap<String, Object>), &str> {
    unimplemented!()
}

pub fn sections_parser(input: State<&str>) -> ParseResult<Object, &str> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::combine::*;
    use super::{Object};
    use super::{assignment_parser, boolean_parser, object_parser, section_parser, sections_parser, single_object_parser, string_parser, struct_parser, title_parser, wierd_exception};

    const true_object : Object = Object::Boolean(true);

    fn test<A: Eq, F: Fn(State<&str>) -> ParseResult<A, &str>>(my_parser : F, input : &str, output : A) {
        let result = parser(my_parser).parse(input);
        match result {
            Ok((result, rest)) => {
                assert!(result == output);
                assert!(rest == "");
            },
            _                  => assert!(false)
        }
    }

    #[test]
    fn test_title_parser() {
        test(title_parser, "[hello]", "hello".to_string());
    }

    #[test]
    fn test_string_parser() {
        test(string_parser, "\"hello \\\"world\\\"\"", "hello \"world\"".to_string());
    }

    #[test]
    fn test_boolean_parser() {
        test(boolean_parser, "true", true_object);
    }

    #[test]
    fn test_wierd_exception_parser() {
        let wierd_object : Object = Object::RandomText("wierd".to_string());
        test(wierd_exception, "$$string", wierd_object);
    }

    #[test]
    fn test_single_object_parser() {
        let wierd_object : Object = Object::RandomText("wierd".to_string());
        test(single_object_parser, "123", Object::IntObject(123));
        test(single_object_parser, "true", true_object);
        test(single_object_parser, "\"string\"", Object::String("string".to_string()));
        test(single_object_parser, "$$wierd", wierd_object);
    }

    #[test]
    fn test_struct_parser() {
        test( struct_parser
            , "{col1, col2
               1, 2
               \"hello\", \"world\"
               true, false
               }"
            , ( vec!("col1".to_string(), "col2".to_string())
              , vec!(vec!(Object::IntObject(1), Object::IntObject(2)),
                     vec!(Object::String("hello".to_string()), Object::String("world".to_string())),
                     vec!(true_object, Object::Boolean(false)))
              )
            )
    }

    #[test]
    fn test_object_parser() {
        test(object_parser,
             "1, 2, 3",
             Object::VecObject(vec!(Object::IntObject(1), Object::IntObject(2), Object::IntObject(3))));
    }

    #[test]
    fn test_assignment_parser() {
        test(assignment_parser,
             "test = 1",
             ("test".to_string(), Object::IntObject(1)));
    }

    #[test]
    fn test_section_parser() {
        let mut hash_map = HashMap::new();
        hash_map.insert("test1".to_string(), Object::IntObject(1));
        hash_map.insert("test2".to_string(), Object::String("hello world".to_string()));
        hash_map.insert("test3".to_string(), true_object);
        test(section_parser,
             "[test]
              test1 = 1
              test2 = \"hello world\"
              test3 = true",
             ("test".to_string(), hash_map));
    }
}
