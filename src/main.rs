extern crate motivations;
extern crate pencil;
extern crate pick_one;

use pencil::Pencil;

mod templates;


fn main() {
    let mut app = Pencil::new("./src");
    app.register_template("motivation.html");
    app.get("/", "motivation", templates::motivation);
    let host: &str = "127.0.0.1";
    const PORT: u32 = 7878;
    let inet4_addr = format!("{}:{}", host, PORT);
    println!("* Running Rust app on {}...", inet4_addr);
    app.run(inet4_addr);
}
