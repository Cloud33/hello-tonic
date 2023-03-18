use pb::hello::{greeter_server::Greeter, HelloRequest, HelloReply};
use tonic::{Response, Request, Status};


pub mod pb;

// client






// server
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);
 
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // 由于gRPC请求和响应中的字段都是私有的，所以需要使用 .into_inner()
        };
 
        Ok(Response::new(reply)) // 发回格式化的问候语
    }
}