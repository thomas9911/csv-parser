extern crate csv_parser;

use csv_parser::{convert_csv, parse_csv};

fn main() -> Result<(), String> {
    let txt = r#"
12,2,3,2,3,1,"oke geinig, oke, topper"
1,2,3,2,3,1,2,'hallo'
1,2,3,2, "CHEESE",3.4,1,2.2
2
"#;

    let csv = parse_csv(txt)?;
    println!("{}", convert_csv(csv));
    Ok(())
}
