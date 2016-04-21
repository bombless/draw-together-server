#[macro_use] extern crate nickel;
extern crate rand;
extern crate rustc_serialize;
extern crate time;


use rustc_serialize::json;
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
            json::encode(&tokens.lock().unwrap().keys().map(ToString::to_string).collect::<Vec<_>>()).unwrap()
        }
    });

    let last_update_time = Mutex::new(time::now());

    server.utilize(router! {
        get "/testList" => |_req, _res| {
            let mut guard = last_update_time.lock().unwrap();
            if *guard + time::Duration::seconds(5) > time::now() {
               "[]".to_string()
            } else {
                *guard = time::now();
                let (x, y) = rand::random::<(u8, u8)>();
                format!("[{{\"type\":1,\"radius\":0.3,\"x\":{},\"y\":{}}}]", x as f32 / 256.0, y as f32 / 256.0)
            }
        }
    });

    server.listen("0.0.0.0:6767");
}
