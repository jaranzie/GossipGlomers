use async_trait::async_trait;
use maelstrom::{Node, Runtime, Result, protocol::{Message, message, MessageBody}, done};
use std::sync::{Arc, Mutex};
use serde_json::{Map, Value, json};
use std::collections::HashMap;
#[derive(Default)]
struct EchoNode {
    seen_values: Mutex<Vec<i64>>,
    neighbors: Vec<String>
}



// Maybe build graph from topology, and use that to determine the number of times to propagate a message

#[async_trait]
impl Node for EchoNode {
    async fn process(&self, runtime: Runtime, request: Message) -> Result<()> {

        match request.get_type() {
            "broadcast" => {
                let v = request.body.extra.get("message").unwrap();
                {
                    let mut lk = self.seen_values.lock().unwrap();
                    if !lk.contains(&v.as_i64().unwrap()) {
                        lk.push(v.as_i64().unwrap());
                    }
                }
                let mut response_body = request.body.clone().with_type("broadcast_ok");
                response_body.extra.clear();
                let res = runtime.reply(request, response_body).await;


                let mut extra = Map::from([("")])
                let broadcast_body = MessageBody::from_extra()
                let mut new_request = message(runtime.node_id(), "", new_body).unwrap();



                for node in self.neighbors.iter() {
                    let m = new_request.clone();
                }

                return res;
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
                let mut my_neighbors = v.get(me).unwrap()
                    .as_array().unwrap()
                    .iter().map(|x| x.as_str().unwrap_or("").to_owned()).collect::<Vec<String>>();
                let me_number = me[1..].parse::<i32>().unwrap();
                for i in 1..me_number as usize {
                    let prior = format!("n{}", i);
                    let prior_neightbors = v.get(&prior).unwrap().as_array().unwrap();
                    for node in prior_neightbors.iter() {
                        let index = my_neighbors.iter().position(|x| x == node.as_str().unwrap());
                        if let Some(idx) = index {
                            my_neighbors.remove(idx);
                        }
                    }

                }

                
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
