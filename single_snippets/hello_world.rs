// Lifetime matter
fn skip_prefix <'line_valid>(line: &'line_valid str, prefix: &str) -> &'line_valid str {
    let (s1, s2) = line.split_at(prefix.len());
    // missing lifetime specifier
    s2

}

fn main() {
    let line =  "lang:en=Hello World!";
    let v;
    {
        let p = "lang:en=";
        v = skip_prefix(line, p);
    }
    println!("{}", v)
}