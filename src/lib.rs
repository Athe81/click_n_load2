#[macro_use]
extern crate nickel;

use nickel::{ Nickel, HttpRouter };

pub fn listen() {
    let mut server = Nickel::new();

    server.get("/flash/?", middleware! {
        "JDownloader"
    });

    server.get("/jdcheck.js/?", middleware! {
        "jdownloader=true; var version='9.581;'"
    });

    server.listen("127.0.0.1:9666");
}
