use md5;
fn main() {
    let secret_key: String = String::from("bgvyzdsv");
    let mut start_value: i32 = 1;
    let mut str_first_six_chars: String = String::new();
    while str_first_six_chars != "000000" {
        start_value += 1;
        let concat_value: String = format!("{secret_key}{start_value}");
        let str_hash: String = format!("{:x}", md5::compute(concat_value));
        str_first_six_chars = str_hash[..6].to_string();
    }
    println!("{}", start_value);
}
