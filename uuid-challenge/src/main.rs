use async_trait::async_trait;
use maelstrom::{Node, Runtime, Result, protocol::Message, done};
use std::sync::Arc;
use uuid::Uuid;
#[derive(Clone, Default)]
struct UuidNode;

#[async_trait]
impl Node for UuidNode {
    async fn process(&self, runtime: Runtime, request: Message) -> Result<()> {

        if request.get_type() == "generate" {
            let mut response_body = request.body.clone().with_type("generate_ok");
            response_body.extra.insert("id".to_string(), serde_json::to_value(Uuid::new_v4().to_string()).unwrap());
            return runtime.reply(request, response_body).await;
        }
        
        done(runtime, request)
    }
}

fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let node = Arc::new(UuidNode::default());
    Runtime::new().with_handler(node).run().await
}
