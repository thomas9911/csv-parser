use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CsvParser;

#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Number(f64),
}


pub fn parse_csv(txt: &str) -> Result<Vec<Vec<Value>>, String> {
    let successful_parse = CsvParser::parse(Rule::csv, txt);

    let parsed = match successful_parse {
        Ok(x) => x,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };

    let mut table: Vec<Vec<Value>> = vec![];

    for parse in parsed {
        let mut line: Vec<Value> = vec![];
        match parse.as_rule() {
            Rule::values => {
                for r in parse.into_inner() {
                    match r.as_rule() {
                        Rule::number => line.push(Value::Number(r.as_str().parse().unwrap())),
                        Rule::string => line.push(Value::String(String::from(r.as_str()))),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        if !line.is_empty() {
            table.push(line);
        }
    }

    Ok(table)
}

pub fn convert_csv(t: Vec<Vec<Value>>) -> String {
    let mut csv = String::new();
    for line in t {
        for v in line {
            match v {
                Value::Number(x) => csv.push_str(&x.to_string()),
                Value::String(x) => csv.push_str(&format!("{:?}", x)),
            }
            csv.push(',');
        }
        csv.pop();
        csv.push('\n');
    }
    csv.pop();
    csv
}


#[test]
fn parse_and_convert_are_invertable() {
    let txt = r#"12,2,3,2,3,1,"1 testing, test, question"
1,2,3,2,3,1,2,"test"
1,2,3,2,"CHEESE",3.4,1,2.2
2"#;

    let csv = parse_csv(txt).unwrap();
    assert_eq!(txt, convert_csv(csv));
}


#[test]
fn parse_empty_string() {
    let txt = r#""#;
    let csv = parse_csv(txt).unwrap();
    let expected: Vec<Vec<Value>> = vec![];
    assert_eq!(csv, expected);
}

#[test]
fn parse_accepts_different_quotes() {
    let txt = r#"
        12,2, 3
        12,2,'t'
        12,2,"1"
        "testing"
    "#;
    let csv = parse_csv(txt).unwrap();
    let expected = vec![
        vec![Value::Number(12.0), Value::Number(2.0), Value::Number(3.0)],
        vec![
            Value::Number(12.0),
            Value::Number(2.0),
            Value::String(String::from("t")),
        ],
        vec![
            Value::Number(12.0),
            Value::Number(2.0),
            Value::String(String::from("1")),
        ],
        vec![Value::String(String::from("testing"))],
    ];
    assert_eq!(csv, expected);
}

#[test]
fn format() {
    let csv = vec![
        vec![Value::Number(12.0), Value::Number(2.0), Value::Number(3.0)],
        vec![
            Value::Number(12.0),
            Value::Number(2.0),
            Value::String(String::from("t")),
        ],
        vec![
            Value::Number(12.0),
            Value::Number(2.0),
            Value::String(String::from("1")),
        ],
        vec![Value::String(String::from("testing"))],
    ];

    let expected = r#"12,2,3
12,2,"t"
12,2,"1"
"testing""#;
    assert_eq!(expected, convert_csv(csv));
}