// Basic Regular Expressions
// re.is_match(text)

use regex::Regex;

fn re_check(re: &str, text: &str) -> bool {
    let re: Regex = Regex::new(re).unwrap();
    re.is_match(text)
}

fn show_re_test(re: &str, text: &str) {
    if re_check(re, text) {
        print!("\n  RegEx: {:?} matches text: {:?}", re, text)
    }    
    else {
        print!("\n  RegEx: {:?} does not match text: {:?}", re, text)
    }
}

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2014-01-01"));

    let re = "abc";
    let text1 = "123abc987";
    show_re_test(re, text1);
    let text2 = "123000987";
    show_re_test(re, text2);
    
    let re = r"([a-z]+)|([A-Z][A-Z])";
    let text3 = "???abc";
    show_re_test(re, text3);
    let text4 = "???A@@@";
    show_re_test(re, text4);
    let text4 = "???AK@@@";
    show_re_test(re, text4);
    let text5 = r"123";
    show_re_test(re, text5);
}