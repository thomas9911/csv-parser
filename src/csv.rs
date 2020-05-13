use pest::Parser;

#[derive(Parser)]
#[grammar = "official_csv.pest"]
pub struct OfficialCsvParser;

pub fn parse_csv(txt: &str) -> Result<Vec<Vec<String>>, String> {
    let successful_parse = OfficialCsvParser::parse(Rule::file, txt);
    let parsed = match successful_parse {
        Ok(x) => x,
        Err(e) => {
            return Err(format!("{}", e));
        }
    };
    let mut table: Vec<Vec<String>> = vec![];
    for parse in parsed {
        match parse.as_rule() {
            Rule::record | Rule::header => {
                let mut line = vec![];
                for p in parse.into_inner() {
                    line.push(String::from(p.as_str()));
                }
                table.push(line);

            }
            _ => {}
        }
    }
    Ok(table)
}


pub fn convert_csv(t: Vec<Vec<String>>) -> String {
    let mut csv = String::new();
    for line in t {
        for v in line {
            csv.push_str(&v);
            csv.push(',');
        }
        csv.pop();
        csv.push('\n');
    }
    csv.pop();
    csv
}

pub struct TableOptions<'a> {
    pub corner: &'a str,
    pub upper: &'a str,
    pub bottom: &'a str,
    pub side: &'a str,
    pub between: &'a str,
    pub extra_pad: usize,
}

impl<'a> Default for TableOptions<'a> {
    fn default() -> TableOptions<'a> {
        TableOptions {
            corner: "+",
            upper: "-",
            bottom: "-",
            side: "|",
            between: "|",
            extra_pad: 12,
        }
    }
}

impl<'a> TableOptions<'a> {
    pub fn new(
        corner: &'a str,
        upper: &'a str,
        bottom: &'a str,
        side: &'a str,
        between: &'a str,
        extra_pad: usize,
    ) -> TableOptions<'a> {
        TableOptions {
            corner,
            upper,
            bottom,
            side,
            between,
            extra_pad,
        }
    }
}

pub fn print_table(t: Vec<Vec<String>>, config: Option<TableOptions>) -> String {
    let padded_table = pad_table(t);
    let column_lengths = get_table_col_len(&padded_table);
    create_table(padded_table, column_lengths, config)
}

pub fn print_html_table(t: Vec<Vec<String>>) -> String {
    let padded_table = pad_table(t);
    create_html_table(padded_table)
}

pub fn print_padded_table(t: Vec<Vec<String>>) -> String {
    let padded_table = pad_table(t);
    convert_csv(padded_table)
}

fn create_html_table(t: Vec<Vec<String>>) -> String {
    let mut result = String::new();
    for line in t {
        result.push_str("<tr>");
        for v in line.iter() {
            result.push_str("<td>");
            result.push_str(v);
            result.push_str("</td>");
        }
        result.push_str("</tr>");
    }
    result
}

fn create_table(t: Vec<Vec<String>>, lengths: Vec<usize>, config: Option<TableOptions>) -> String {
    let config = match config {
        Some(x) => x,
        None => TableOptions::default(),
    };

    let mut result = String::new();
    for l in lengths.iter() {
        result.push_str(config.corner);
        for _i in 0..(*l + config.extra_pad) {
            result.push_str(config.upper);
        }
    }
    result.push_str(config.corner);

    for line in t {
        result.push('\n');
        result.push_str(config.side);
        for (v, i) in line.iter().zip(lengths.iter()) {
            result.push_str(&format!("{:>width$}", v, width = i + config.extra_pad));
            result.push_str(config.between);
        }
        result.pop();
        result.push_str(config.side);
    }
    result.push('\n');
    for l in lengths.iter() {
        result.push_str(config.corner);
        for _i in 0..(*l + config.extra_pad) {
            result.push_str(config.bottom);
        }
    }
    result.push_str(config.corner);
    result
}

fn pad_table(t: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut col_len = 0;

    for i in t.iter() {
        if i.len() > col_len {
            col_len = i.len();
        }
    }

    let mut new_table = vec![];
    for i in t {
        let mut line = i.clone();
        while line.len() < col_len {
            line.push(String::from(""));
        }
        new_table.push(line);
    }
    new_table
}

fn get_table_col_len(t: &Vec<Vec<String>>) -> Vec<usize> {
    let mut col_lens = vec![];
    for i in t {
        for (k, j) in i.iter().enumerate() {
            match col_lens.get(k) {
                Some(x) => {
                    if x < &j.len() {
                        col_lens[k] = j.len()
                    }
                }
                None => col_lens.push(j.len()),
            }
        }
    }
    col_lens
}

macro_rules! s {
    ($x:expr) => {
        String::from($x)
    };
}

#[test]
fn parse_empty_string() {
    let txt = r#""#;
    let csv = parse_csv(txt).unwrap();
    let expected: Vec<Vec<String>> = vec![vec![String::from("")]];
    assert_eq!(csv, expected);
}

#[test]
fn parse() {
    let txt = r#"1,2,3,hallo
test, cheese,chocolate,"parsing some csv's"
"#;
    let csv = parse_csv(txt).unwrap();
    let expected: Vec<Vec<String>> = vec![
        vec![s!("1"), s!("2"), s!("3"), s!("hallo")],
        vec![
            s!("test"),
            s!(" cheese"),
            s!("chocolate"),
            s!("\"parsing some csv's\""),
        ],
        vec![s!("")],
    ];
    assert_eq!(csv, expected);
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
fn test_print_table() {
    let table: Vec<Vec<String>> = vec![
        vec![s!("1"), s!("2"), s!("3"), s!("hallo")],
        vec![
            s!("test"),
            s!(" cheese"),
            s!("chocolate"),
            s!("\"parsing some csv's\""),
        ],
        vec![s!("")],
    ];

    let expected = String::from(r#"+----------------+-------------------+---------------------+--------------------------------+
|               1|                  2|                    3|                           hallo|
|            test|             cheese|            chocolate|            "parsing some csv's"|
|                |                   |                     |                                |
+----------------+-------------------+---------------------+--------------------------------+"#);
    assert_eq!(print_table(table, None), expected);

}


#[test]
fn test_print_table_with_config() {
    let config = TableOptions::new("0", "^", "v", "#", ";", 2);
    let table: Vec<Vec<String>> = vec![
        vec![s!("1"), s!("2"), s!("3"), s!("hallo")],
        vec![
            s!("test"),
            s!(" cheese"),
            s!("chocolate"),
            s!("\"parsing some csv's\""),
        ],
        vec![s!("")],
    ];

    let expected = String::from(r##"0^^^^^^0^^^^^^^^^0^^^^^^^^^^^0^^^^^^^^^^^^^^^^^^^^^^0
#     1;        2;          3;                 hallo#
#  test;   cheese;  chocolate;  "parsing some csv's"#
#      ;         ;           ;                      #
0vvvvvv0vvvvvvvvv0vvvvvvvvvvv0vvvvvvvvvvvvvvvvvvvvvv0"##);
    assert_eq!(print_table(table, Some(config)), expected);
}