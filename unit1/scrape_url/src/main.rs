fn main() {
    for arg in std::env::args() {
        println!("{}", arg);
    }
    let args: Vec<String> = std::env::args().collect();
    // let url = "http://www.rust.org/";
    // let output = "rust.md";
    if args.len() <= 2 {
        println!("Sorry please input the url and output");
        return;
    }
    let url = &args[1];
    let output = &args[2];
    println!("Fecthing url :{}", url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("Converting html to markdown...");
    let md = html2md::parse_html(&body);

    std::fs::write(output, md.as_bytes()).unwrap();

    println!("Converted markdown has been saved in {}", output);
}
