#[macro_use]
extern crate nickel;
extern crate url;
extern crate rustc_serialize;
extern crate crypto;

use std::io::Read;
use std::string::String;
use std::process::Command;
use std::ops::Deref;

use nickel::{ Nickel, HttpRouter };

use rustc_serialize::base64::FromBase64;
use rustc_serialize::hex::FromHex;

use crypto::{ buffer, aes, blockmodes };
use crypto::buffer::{WriteBuffer, ReadBuffer};
use crypto::symmetriccipher::{ Decryptor};

pub fn listen() {
    let mut server = Nickel::new();

    server.get("/flash/?", middleware! {
        "JDownloader"
    });

    server.get("/jdcheck.js/?", middleware! {
        "jdownloader=true; var version='9.581;'"
    });

    server.post("/flash/addcrypted2/?", middleware! { |req, res|
        let mut form_data = String::new();
        req.origin.read_to_string(&mut form_data).unwrap();

        let data = url::form_urlencoded::parse(form_data.as_bytes());

        let mut passwords = String::new();
        let mut source = String::new();
        let mut jk = String::new();
        let mut crypted = String::new();

        for (key, value) in data {
            match key.as_ref() {
                "passwords" => passwords = value,
                "source" => source = value,
                "jk" => jk = value,
                "crypted" => crypted = value,
                _ => {},
            }
        }

        // TODO: check if external command exist
        jk.push_str(" console.log(f())");

        let key = Command::new("js")
            .arg("-e")
            .arg(&jk)
            .output()
            .unwrap();


        let key = String::from_utf8(key.stdout)
            .unwrap()
            .trim()
            .from_hex()
            .unwrap();

        let crypted = crypted.from_base64().unwrap();

        let mut out = [0; 4096];
        let mut reader = buffer::RefReadBuffer::new(&crypted);
        let mut writer = buffer::RefWriteBuffer::new(&mut out);
        
        let mut dec = aes::cbc_decryptor(
            aes::KeySize::KeySize128,
            key.deref(),
            key.deref(),
            blockmodes::NoPadding,
        );

        let mut result = Vec::new();
        loop {
            dec.decrypt(&mut reader, &mut writer, true).unwrap();
            if writer.is_empty() {
                break;
            }
            result.extend_from_slice(writer.take_read_buffer().take_remaining());
        };

        println!("Passwords: {:?}\nSource: {:?}", passwords, source);
        println!("{}", String::from_utf8(result).unwrap());
    });

    server.listen("127.0.0.1:9666");
}
