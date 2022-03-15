fn main() {
    let number = 1;
    let the_str = "Foo";
    let passed = [passing(padded_name, char_vals_id, number, the_str)
        , passing(gen_name_from_int, gen_id_from_string, number, the_str)];
    println!("{:?}", passed)
}

#[derive(Debug)]
struct Passed {
    id: usize,
    name: String,
}


fn passing(add_name: impl Fn(i32) -> String, add_id: impl Fn(&str) -> usize, the_number: i32, the_string: &str) -> Passed {
    Passed {
        id: add_id(the_string),
        name: add_name(the_number),
    }
}

fn gen_name_from_int(number: i32) -> String {
    match number {
        1 => "Roar".to_string(),
        2 => "Reidar".to_string(),
        _ => "Not sett".to_string()
    }
}

fn gen_id_from_string(str: &str) -> usize {
    str.len()
}

fn padded_name(number: i32) -> String {
    format!("{:05}", number)
}

//Returned value is immutable
fn char_vals_id(inn: &str) -> usize {
    let mut count: usize = 0;
    for c in inn.chars() {
        count += c.len_utf16()
    };
    count
}
