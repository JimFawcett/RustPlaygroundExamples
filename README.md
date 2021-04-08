# RustPlaygroundExamples
A set of links to playground examples that illustrate small bits of Rust syntax

**Playground links to Rust syntax examples**<br />

<table>
    <tr>
        <td style="border-right;">
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=2e153e600daf37d3909ca2d6b5950f56">ownership</a>
        </td>
        <td>
        Ownership violations governed by use
        </td>
    </tr>
    <tr>
        <td>
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e41f33b89b939afa8476aa84981e4345">functions</a>
        </td>
        <td>
           pass by value and by reference 
        </td>
    </tr>
    <tr>
        <td>
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0e149a761bf748f4a2b9f7acc120e816">structs</a>
        </td>
        <td>
           struct with constructor and methods 
        </td>
    </tr>
    <tr>
        <td>
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=7dd8fee11e0afe4f1772b6eaa63031b8">generic struct</a>
        </td>
        <td>
            generic struct with bounds
        </td>
    </tr>
    <tr>
        <td>
<a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=53030e9e8e06413538dfa61f3d02f2e8">functions accepting iterator</a>
        </td>
        <td>
            explicit use of iterator in loop, implicit use with for loop
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=f375f3792f84e55cb65d5de3c7d01956">iterator for custom type
        </td>
        <td>
            custom type returns iterator over owned collection
        </td>
    <tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=527cbf992f6f8991137b105b25dcd7d2">custom iterator
        </td>
        <td>
            implement custom iterator for custom type
        </td>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d871a66bd82f6001ee6cb75a5572a975">interation over collections</a>
        </td>
        <td>
            iterating over array, String, Vec, VecDeq, and HashMap
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0fd39d1d6629d52d42e225f292d06db4">into_iter</a>
        </td>
        <td>
            compares iter() with into_iter()
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=c91bdd1431608e101471f3e43e54a73a">generic function and struct</a>
        </td>
        <td>
            displays type using std::any::type_name
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=784213671fd9d152118f8ca17ad29feb">mut struct copy and move</a>
        </td>
        <td>
           Illustrates use (and non-use) of derived Copy trait 
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=e609d2e8baf89018de7d236b2c946020">custom enumertion</a>
        </td>
        <td>
            illustrates matching with custom enumeration
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d8ff223b3836c50e1d69e0fcce5b6769">non-enum matching</a>
        </td>
        <td>
            matching chars: digit, lower case, uppercase, all else
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=5e92f5a601fee0811f8b6ec57ab11cb9">file error handling</a>
        </td>
        <td>
            open for read, write, and read to string
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=6c4eca51d0547462043a8e00c1d063c5">error bubbling
        </td>
        <td>
            illustrates file operations with error bubbling
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=fc78d87ca01bce284c538ca9d59973a5>string conversions</a>
        </td>
        <td>
            convert between String, &str, PathBuf, &Path
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&code=%23!%5Ballow(dead_code)%5D%0A%23!%5Ballow(unused_variables)%5D%0Ause%20std%3A%3Aresult%3A%3A*%3B%0A%0Afn%20demo_result%3C%27a%3E(p%3A%20bool)%20-%3E%20Result%3C%26%27a%20str%2C%20%26%27a%20str%3E%20%7B%0A%20%20%20%20print!(%22%5Cn%20%20value%20of%20input%20predicate%20is%20%7B%7D%22%2C%20p)%3B%0A%20%20%20%20if%20p%20%7B%0A%20%20%20%20%20%20%20%20Ok(%22it%27s%20ok%22)%0A%20%20%20%20%7D%20%0A%20%20%20%20else%20%7B%0A%20%20%20%20%20%20%20%20Err(%22not%20ok%22)%0A%20%20%20%20%7D%0A%7D%0A%0Afn%20demo_option%3C%27a%3E(p%3Abool)%20-%3E%20Option%3C%26%27a%20str%3E%20%7B%0A%20%20%20%20print!(%22%5Cn%20%20value%20of%20input%20predicate%20is%20%7B%7D%22%2C%20p)%3B%0A%20%20%20%20if%20p%20%7B%0A%20%20%20%20%20%20%20%20Some(%22something%20just%20for%20you!%22)%0A%20%20%20%20%7D%0A%20%20%20%20else%20%7B%0A%20%20%20%20%20%20%20%20None%0A%20%20%20%20%7D%0A%7D%0Afn%20title(msg%3A%26str)%20%7B%0A%20%20%20%20print!(%22%5Cn%20%20--%20%7B%7D%20--%5Cn%22%2C%20msg)%3B%0A%7D%0A%0Afn%20main()%20%7B%0A%20%20%20%20%2F%2F%20use%20display%3A%3A%7B*%7D%3B%0A%0A%20%20title(%22demo%20Result%22)%3B%0A%20%20print!(%22%5Cn--%20using%20match%22)%3B%0A%0A%20%20let%20r%20%3D%20demo_result(true)%3B%0A%20%20match%20r%20%7B%0A%20%20%20%20%20%20Ok(rslt)%20%3D%3E%20print!(%22%5Cn%20%20result%20is%20%7B%7D%22%2C%20rslt)%2C%0A%20%20%20%20%20%20Err(rslt)%20%3D%3E%20print!(%22%5Cn%20%20result%20is%20%7B%7D%22%2C%20rslt)%0A%20%20%7D%0A%20%20let%20r%20%3D%20demo_result(false)%3B%0A%20%20match%20r%20%7B%0A%20%20%20%20%20%20Ok(rslt)%20%3D%3E%20print!(%22%5Cn%20%20result%20is%20%7B%7D%22%2C%20rslt)%2C%0A%20%20%20%20%20%20Err(rslt)%20%3D%3E%20print!(%22%5Cn%20%20result%20is%20%7B%7D%22%2C%20rslt)%0A%20%20%7D%0A%20%20print!(%22%5Cn%5Cn--%20using%20expect%22)%3B%0A%0A%20%20let%20r%20%3D%20demo_result(true)%0A%20%20%20%20.expect(%22predicate%20was%20false%22)%3B%0A%20%20print!(%22%5Cn%20%20%20%20result%20is%20%7B%7D%22%2C%20r)%3B%0A%20%20%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%0A%20%20%2F%2F%20uncomment%20to%20see%20panic%0A%20%20%2F%2F%20let%20_r%20%3D%20demo_result(false)%0A%20%20%2F%2F%20%20%20.expect(%22predicate%20was%20false%22)%3B%0A%20%20println!()%3B%0A%0A%20%20title(%22demo%20Option%22)%3B%0A%20%20print!(%22%5Cn--using%20match%22)%3B%0A%0A%20%20let%20r%20%3D%20demo_option(true)%3B%0A%20%20match%20r%20%7B%0A%20%20%20%20%20%20Some(rslt)%20%3D%3E%20print!(%22%5Cn%20%20%20%20%7B%7D%22%2C%20rslt)%2C%0A%20%20%20%20%20%20None%20%3D%3E%20print!(%22%5Cn%20%20%20%20sorry%2C%20nothing%20here%22)%0A%20%20%7D%0A%20%20let%20r%20%3D%20demo_option(false)%3B%0A%20%20match%20r%20%7B%0A%20%20%20%20%20%20Some(rslt)%20%3D%3E%20print!(%22%5Cn%20%20%20%20%7B%7D%22%2C%20rslt)%2C%0A%20%20%20%20%20%20None%20%3D%3E%20print!(%22%5Cn%20%20%20%20sorry%2C%20nothing%20here%22)%0A%20%20%7D%0A%20%20print!(%22%5Cn%5Cn--using%20unwrap%22)%3B%0A%0A%20%20let%20r%20%3D%20demo_option(true).unwrap()%3B%0A%20%20print!(%22%5Cn%20%20%20%20%7B%7D%22%2C%20r)%3B%0A%20%20%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%2F%0A%20%20%2F%2F%20uncomment%20to%20see%20panic%0A%20%20%2F%2F%20let%20_r%20%3D%20demo_option(false).unwrap()%3B%0A%0A%20%20print!(%22%5Cn%5Cn%20%20That%27s%20all%20folks!%5Cn%5Cn%22)%3B%0A%0A%7D">
               Lifetimes
            </a>
        </td>
        <td>
            Result and Option wrapping &str so need lifetime annotation  
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=28b7994c756c00244848b1668030be97">plugin architecture</a>
        </td>
        <td>
            plugin host holds Option of mutable reference to plugin - needs annotation
        </td>
    </tr>
    <tr>
        <td>
            <a href="https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=931f8460fbe875ce03f4e67dee9b304d">host owns plugin
        </td>
        <td>
            eliminate lifetime annotations in plugin architecture by moving plugin into host
        </td>
    </tr>
    <tr>
        <td>
        </td>
        <td>
        </td>
    </tr>
    <tr>
        <td>
        </td>
        <td>
        </td>
    </tr>
    <tr>
        <td>
        </td>
        <td>
        </td>
    </tr>
</table>
