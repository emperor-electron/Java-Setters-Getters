use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("This program requires an input of at least 3 strings, which are expected to be valid Java field names.");
        exit(1)
    }

    let _method_access_mod: String = args[1].clone();
    let data_type: String = args[2].clone();
    let field_name: String = args[3].clone().replace(";", "");

    let mut getter_name = format!("get{name}", name = &field_name[0..1].to_uppercase());
    getter_name = format!("{}{}", getter_name, &field_name[1..]);

    let getter_javadoc = format!(
        "/**
 * @return (insert_description)
 */
"
    );

    let mut setter_name = format!("set{name}", name = &field_name[0..1].to_uppercase());
    setter_name = format!("{}{}", setter_name, &field_name[1..]);

    let setter_javadoc = format!(
        "/**
 * @param (insert_description)
 */
"
    );

    let getter = format!(
        "private {} {} () {{
  return this.{};
}}",
        data_type, getter_name, field_name
    );

    let setter = format!(
        "private void {} ({} value) {{
  this.{} = value;
}}",
        setter_name, data_type, field_name
    );

    println!("{}{}\n", getter_javadoc, getter);
    println!("{}{}\n", setter_javadoc, setter);
}
