pub fn run_in_main () {
    let mut numbers: Vec<i32> = vec![10, 15, 20, 25];
    let mut dub_values: Vec<i32> = vec![];

    for num in &mut numbers {
        dub_values.push(*num * 2);
    }

    let third: Option<&i32> = dub_values.get(2);
    match third {
        Some(value) => println!("The third value is: {:?}", value),
        None => println!("There is no third element."),
    }

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(12.63),
        SpreadsheetCell::Text(String::from("Too many rocks")),
        SpreadsheetCell::Int(97),
        SpreadsheetCell::Float(16.56),
        SpreadsheetCell::Text(String::from("Up on the ledge")),
        SpreadsheetCell::Int(18),
        SpreadsheetCell::Float(1.2),
        SpreadsheetCell::Text(String::from("Reassess the test"))
    ];

    let mut int_col: Vec<i32> = vec![];
    let mut float_col: Vec<f64> = vec![];
    let mut text_col: Vec<String> = vec![];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => int_col.push(*value),
            SpreadsheetCell::Float(value) => float_col.push(*value),
            SpreadsheetCell::Text(value) => text_col.push(value.to_string()),
        }
    }

    println!("Int column: {:?}", int_col);
    println!("Float column: {:?}", float_col);
    println!("Text column: {:?}", text_col);
    println!("Parent Vector: \n{:?}", row);
}