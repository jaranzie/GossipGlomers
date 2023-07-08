use async_trait::async_trait;
use maelstrom::{Node, Runtime, Result, protocol::Message, done};
use std::sync::Arc;
#[derive(Clone, Default)]
struct EchoNode;

#[async_trait]
impl Node for EchoNode {
    async fn process(&self, runtime: Runtime, request: Message) -> Result<()> {

        if request.get_type() == "echo" {
            let response_body = request.body.clone().with_type("echo_ok");
            return runtime.reply(request, response_body).await;
        }
        
        done(runtime, request)
    }
}

fn main() -> Result<()> {
    Runtime::init(try_main())
}

async fn try_main() -> Result<()> {
    let node = Arc::new(EchoNode::default());
    Runtime::new().with_handler(node).run().await
}
