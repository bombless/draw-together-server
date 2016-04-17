#[macro_use] extern crate nickel;
extern crate rand;


use nickel::Nickel;
use rand::random;

fn main() {
    use std::collections::HashMap;
    use std::sync::{ Arc, Mutex };
    

    let tokens = Arc::new(Mutex::new(HashMap::<u64, ()>::new()));
    let tokens_write = tokens.clone();
    let mut server = Nickel::new();

    server.utilize(router! {
        get "/new" => |_req, _res| {
            let token = random();
            tokens_write.lock().unwrap().insert(token, ());
            token.to_string()
        }
    });



    server.utilize(router! {
        get "/list" => |_req, _res| {
            tokens.lock().unwrap().keys().map(ToString::to_string).collect::<Vec<_>>().join(",")
        }
    });


    server.listen("0.0.0.0:6767");
}
