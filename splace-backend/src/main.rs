use std::sync::Arc;
use std::time::Duration;
use stormplace::*;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};

use crate::stormplace::PublicId;

mod canvas;

mod stormplace {
    include!("stormplace.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let playground = canvas::Playground::new(1000, 1000);
    playground.set_pixel((0, 0), 5).await;
    // dbg!(playground.get_all_pixels().await);

    let playground_counter = Arc::new(playground);
    let palyground_writer = Arc::clone(&playground_counter);

    for _ in 0..2 {
        tokio::spawn(random_reads(Arc::clone(&playground_counter)));
    }

    tokio::spawn(async move {
        let mut counter = 0;
        loop {
            palyground_writer.set_pixel((0, 0), counter).await;
            tokio::time::sleep(Duration::new(5, 0)).await;
            counter += 1;
        }
    });

    let addr = "[::1]:50051".parse().unwrap();
    let server = StormplaceServer {
        playground: playground_counter,
    };
    println!("Stormplace server listening on {}", addr);

    Server::builder()
        .add_service(stormplace_server::StormplaceServer::new(server))
        .serve(addr)
        .await?;

    Ok(())
}

struct StormplaceServer {
    playground: Arc<canvas::Playground>,
}

#[tonic::async_trait]
impl stormplace_server::Stormplace for StormplaceServer {
    type StreamChangesStream = ReceiverStream<Result<PixelUpdate, Status>>;
    async fn stream_changes(
        &self,
        request: Request<PublicId>,
    ) -> Result<Response<Self::StreamChangesStream>, Status> {
        let playground = Arc::clone(&self.playground);
        let mut subscriber = playground.subscribe();
        let (tx, rx) = mpsc::channel(256);

        println!("User {} started listening!", request.get_ref().name);

        tokio::spawn(async move {
            loop {
                let update = subscriber.recv().await;
                let to_send = match update {
                    Ok(event) => Ok(event),
                    Err(e) => Err(Status::new(Code::Unknown, e.to_string())),
                };
                let mut terminate = to_send.is_err();
                if let Err(e) = tx.send(to_send).await {
                    println!("Error sending to client: {}", e);
                    terminate = true;
                }
                if terminate {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type GetCanvasStateOnceStream = ReceiverStream<Result<PixelUpdate, Status>>;

    async fn get_canvas_state_once(
        &self,
        _request: tonic::Request<PublicId>,
    ) -> Result<tonic::Response<Self::GetCanvasStateOnceStream>, tonic::Status> {
        let playground = Arc::clone(&self.playground);

        let (tx, rx) = mpsc::channel(256);
        tokio::spawn(async move {
            for update in playground.get_pixels_as_updates().await {
                tx.send(Ok(update)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn paint_pixel(
        &self,
        request: tonic::Request<PixelPaintRequest>,
    ) -> Result<tonic::Response<PixelPaintResponse>, tonic::Status> {
        let playground = Arc::clone(&self.playground);
        let req = request.get_ref();
        playground
            .set_pixel((req.x, req.y), req.color as canvas::Color)
            .await;

        return Ok(Response::new(PixelPaintResponse { success: true }));
    }

    async fn get_metadata(
        &self,
        _request: tonic::Request<CanvasMetadataRequest>,
    ) -> Result<tonic::Response<CanvasMetadata>, tonic::Status> {
        let playground = Arc::clone(&self.playground);
        Ok(Response::new(playground.get_metadata()))
    }
}

async fn random_reads(playground: Arc<canvas::Playground>) {
    loop {
        tokio::time::sleep(Duration::new(10, 0)).await;
        println!(
            "Pixel at (0, 0) is {}",
            playground.get_pixel_at_location(&(0, 0)).await
        )
    }
}
