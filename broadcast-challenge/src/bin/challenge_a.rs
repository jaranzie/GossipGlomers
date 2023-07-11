use async_trait::async_trait;
use maelstrom::{Node, Runtime, Result, protocol::Message, done};
use std::sync::{Arc, Mutex};
use serde_json::{Map, Value, json};
#[derive(Default)]
struct EchoNode {
    seen_values: Mutex<Vec<i64>>
}

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
