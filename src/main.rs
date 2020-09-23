extern crate websocket;

use std::thread;
use websocket::sync::Server;
use websocket::OwnedMessage;

// struct User {
//     id: u64,
//     ip: SocketAddr
// }

// struct AppState {
//     users: Vec<User>,
//     last_id: u64
// }

// impl AppState {
//     fn last_id(&self) -> u64 {
//         self.last_id
//     }

//     fn user_by_id(&self, id: u64) -> &User {
//         let user = self.users.iter().find(|user| user.id == id).unwrap();
//         user
//     }

//     fn user_by_ip(&self, ip: SocketAddr) -> &User {
//         let user = self.users.iter().find(|user| user.ip == ip).unwrap();
//         user
//     }

//     fn remove_user_by_id(&mut self, id: u64) {
//         let mut user_index = 0;
//         for (i, user) in self.users.iter().enumerate() {
//             if user.id == id {
//                 user_index = i;
//                 break;
//             }
//         }

//         self.users.remove(user_index);
//     }

//     fn last_user(&self) -> &User {
//         let user = self.users.last().unwrap();
//         user
//     }

//     fn create_user(&mut self, ip: SocketAddr) -> &User {
//         let mut id = self.last_id();
//         id = id + 1;

//         let user = User {
//             id: id,
//             ip,
//         };

//         self.users.push(user);
//         self.last_id = id;

//         let user = self.last_user();
//         user
//     }
// }

fn main() {
    let ws_server = Server::bind("127.0.0.1:2727").unwrap();

    // let _app_state = AppState {
    //     users: vec![],
    //     last_id: 0,
    // };

    for request in ws_server.filter_map(Result::ok) {
        thread::spawn(|| {
            if !request.protocols().contains(&"rust-websocket".to_string()) {
				request.reject().unwrap();
				return;
            }
            let client = request.use_protocol("rust-websocket").accept().unwrap();

            let ip = client.peer_addr().unwrap();
            println!("Connection from {}", ip);
            
            let (mut receiver, mut sender) = client.split().unwrap();

            for message in receiver.incoming_messages() {
				let message = message.unwrap();

				match message {
					OwnedMessage::Close(_) => {
						let message = OwnedMessage::Close(None);
						sender.send_message(&message).unwrap();
                        println!("Client {} disconnected", ip);
						return;
					}
					OwnedMessage::Ping(ping) => {
						let message = OwnedMessage::Pong(ping);
						sender.send_message(&message).unwrap();
					}
					_ => sender.send_message(&message).unwrap(),
				}
			}
        });
    }
}