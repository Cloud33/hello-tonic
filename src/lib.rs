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
 
        let reply = match request.metadata().get("inner") {
            Some(_) =>HelloReply {
                message: format!("Hello Inner {}!", request.into_inner().name).into(),
            },
            _ => HelloReply {
                message: format!("Hello {}!", request.into_inner().name).into(),
            },
        };
        // 客户端可以捕捉 grpc 错误
        Err(Status::invalid_argument("name is not a valid"))
        //Ok(Response::new(reply)) 
    }
}