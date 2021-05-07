/* -- demonstrating String iterators --*/
fn main() {
    print!("\n  -- demo string iterators --");
    print!("\n  size of char is {}\n", std::mem::size_of::<char>());
    let s = String::from("a string");
    print!("\n  -- iterating over vec of chars --");
    let v: Vec<char> = s.chars().collect();
    let iter = v.iter();
    print!{"\n  {:?}", v[0]};
    for item in iter.skip(1) {
        print!{", {:?}", item};
    }
    
    print!("\n\n  -- iterating by ref over String --");
    let mut iter = s.chars();
    if let Some(ch) = iter.by_ref().nth(0) {
        print!("\n  {:?}", ch);
        for item in iter.by_ref() {
            print!(", {:?}", item)
        }
    }

    print!("\n\n  -- iterating over String again --");
    // yields nothing since there is no way to reset iterator
    if let Some(ch) = iter.nth(0) {
        print!("\n  {:?}", ch);
        for item in iter {
            print!(", {:?}", item)
        }
    }

    print!("\n\n  -- iterating over String again --");
    let mut iter = s.chars();
    if let Some(ch) = iter.nth(0) {
        print!("\n  {:?}", ch);
        for item in iter {
            print!(", {:?}", item);
        }
    }

    print!("\n\n  -- iterating over String with char_indices --");
    let mut iter = s.char_indices();
    if let Some(ch) = iter.nth(0) {
        print!("\n  {:?}", ch);
        for item in iter {
            print!(", {:?}", item);
        }
    }

    print!("\n\n  -- iterating over String bytes --");
    let mut iter = s.bytes();
    if let Some(ch) = iter.nth(0) {
        print!("\n  {:?}", ch);
        for item in iter {
            print!(", {:?}", item);
        }
    }

    print!("\n\n  -- iterating over &str --");
    let ls: &str = s.as_ref();
    let mut iter = ls.chars();
    if let Some(ch) = iter.nth(0) {
        print!("\n  {:?}", ch);
        for item in iter {
            print!(", {:?}", item)
        }
    }

    print!("\n\n  -- iterating over &str with char_indices --");
    let mut iter = ls.char_indices();
    if let Some(ch) = iter.nth(0) {
        print!("\n  {:?}", ch);
        for item in iter {
            print!(", {:?}", item)
        }
    }

    print!("\n\n  -- iterating over &str bytes --");
    let mut iter = ls.bytes();
    if let Some(ch) = iter.nth(0) {
        print!("\n  {:?}", ch);
        for item in iter {
            print!(", {:?}", item)
        }
    }

    /*-- this fails to compile because iter was consumed --*/
    // print!("\n\n  -- iterating over &str --");
    // let ch = iter.nth(0).unwrap();
    // print!("\n  {:?}", ch);
    // for item in iter {
    //     print!(", {:?}", item)
    // }

    print!("\n\n  -- iterating over non-ASCII String --");
    let mut s = "up up and away ".to_string();
    let ch = '\u{1f680}';  // accelerating rocket
    s.push(ch);
    let iter = s.chars();
    print!("\n  ");
    for item in iter {
        print!("{}", item)
    }

    print!("\n\n  -- iterating over non-ASCII String bytes --");
    // display bytes in hex
    let sb = s.as_bytes();
    let iter = sb.iter();
    print!("\n  ");
    for item in iter {
        print!("{:#x} ", item);
    }
    println!();
    // display bites in binary to check char boundaries
    let iter = sb.iter();
    for item in iter {
        print!("\n  {:#010b}", item);
    }


}