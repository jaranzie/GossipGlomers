use async_trait::async_trait;
use maelstrom::{Node, Runtime, Result, protocol::{Message, MessageBody}, done};
use std::sync::{Arc, Mutex, RwLock};
use serde_json::{Map, json};
use tokio::task;
use serde::{Serialize, Deserialize};
#[derive(Default)]
struct EchoNode {
    seen_values: Mutex<Vec<i64>>,
    neighbors: RwLock<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PropagationMessage {
    // typ
    value: i64,
}

// Maybe build graph from topology, and use that to determine the number of times to propagate a message

#[async_trait]
impl Node for EchoNode {
    async fn process(&self, runtime: Runtime, request: Message) -> Result<()> {
        match request.get_type() {
            "broadcast" => {
                eprintln!("Broadcast from {}", &request.src);
                let v = request.body.extra.get("message").unwrap();
                let already_have:bool;
                {
                    let mut lk = self.seen_values.lock().unwrap();
                    already_have = lk.contains(&v.as_i64().unwrap());
                    if !already_have {
                        lk.push(v.as_i64().unwrap());
                    }
                }

                let mut prop_message = request.clone();
                prop_message.src = runtime.node_id().to_string();
                prop_message.body.in_reply_to = 0;
                prop_message.body.msg_id = 0;
                if !already_have {
                    // eprintln!("Propagating message to {} neighbors", self.neighbors.len());
                    eprintln!("Propagating message {:?}", &prop_message);
                    for node in self.neighbors.read().unwrap().iter() {
                        eprintln!("Propagating message to {}", node);
                        let runtime = runtime.clone();
                        let new_request = prop_message.clone();
                        let dest_node = node.to_owned();
                        task::spawn(async move {
                            let _ = runtime.rpc(dest_node, new_request.body).await;
                        });
                    }
                }
                


                // let mut extra = Map::from([("", "")]);
                // let broadcast_body = MessageBody::from_extra();
                
                let response_body = MessageBody::default().with_type("broadcast_ok").with_reply_to(request.body.msg_id);
                return runtime.reply(request, response_body).await;
            },
            "read" => {
                let mut response_body = request.body.clone().with_type("read_ok");
                let mut extra = Map::new();
                extra.insert("messages".into(), json!(*self.seen_values.lock().unwrap()));
                response_body.extra = extra;
                return runtime.reply(request, response_body).await;
            },
            "topology" => {
                let v = request.body.extra.get("topology")
                    .unwrap()
                    .as_object()
                    .unwrap();
                let me = runtime.node_id();
                let  my_neighbors = v.get(me).unwrap()
                    .as_array().unwrap()
                    .iter().map(|x| x.as_str().unwrap_or("").to_owned()).collect::<Vec<String>>();
                // let me_number = me[1..].parse::<i32>().unwrap();

                for n in my_neighbors.iter() {
                    eprint!("{} ", n);
                }
                eprintln!("");

                *self.neighbors.write().unwrap() = my_neighbors;
                // for i in 1..me_number as usize {
                //     let prior = format!("n{}", i);
                //     let prior_neightbors = v.get(&prior).unwrap().as_array().unwrap();
                //     for node in prior_neightbors.iter() {
                //         let index = my_neighbors.iter().position(|x| x == node.as_str().unwrap());
                //         if let Some(idx) = index {
                //             my_neighbors.remove(idx);
                //         }
                //     }

                // }

                
                let mut response_body = request.body.clone().with_type("topology_ok");
                response_body.extra.clear();
                return runtime.reply(request, response_body).await;
            }
            _ => done(runtime, request) 
        }     
    }
}

fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let node = Arc::new(EchoNode::default());
    Runtime::new().with_handler(node).run().await
}
