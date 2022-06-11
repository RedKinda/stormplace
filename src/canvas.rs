use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::{Arc, RwLockReadGuard};

use tokio::sync::{mpsc, RwLock};
use tokio::sync::broadcast;
use tokio::sync::mpsc::Receiver;
use tokio_stream::wrappers::BroadcastStream;
use tonic::Status;

use crate::{CanvasMetadata, PixelUpdate};

type Location = (u64, u64);
pub(crate) type Color = u8;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    color: Color,
}

impl Default for Pixel {
    fn default() -> Self {
        Self { color: 0 }
    }
}

impl Pixel {
    fn from_color(color: Color) -> Self {
        Pixel {
            color
        }
    }
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Pixel(");
        f.write_str(self.color.to_string().as_str());
        f.write_str(")");
        Ok(())
    }
}

pub struct Playground {
    canvas: RwLock<HashMap<Location, Pixel>>,
    max_width: u64,
    max_height: u64,
    broadcaster: broadcast::Sender<PixelUpdate>
    // listeners: Vec<mpsc::Sender<Result<PixelUpdate, Status>>>
}


impl Playground {
    pub fn new(width: u64, height: u64) -> Self {
        Playground {
            canvas: Default::default(),
            max_width: width,
            max_height: height,

            broadcaster: broadcast::channel(256).0
            // listeners: vec![]
        }
    }

    pub async fn get_pixel_at_location(&self, location: &Location) -> Pixel {
        let canvas = self.canvas.read().await;

        match canvas.get(location) {
            Some(pixel) => pixel.clone(),
            None => Pixel::default()
        }
    }

    pub async fn get_all_pixels(&self) -> Vec<Vec<Option<Pixel>>> {
        let mut res = vec![vec![None; self.max_width as usize]; self.max_height as usize];

        let canvas_reader = self.canvas.read().await;
        for ((x, y), pixel) in canvas_reader.iter() {
            res[*y as usize][*x as usize] = Some(pixel.clone())
        }

        res
    }

    pub async fn get_pixels_as_updates(&self) -> Vec<PixelUpdate> {
        let canvas_reader = self.canvas.read().await;
        return canvas_reader.iter().map(|((x, y), pixel)| PixelUpdate {
            color: pixel.color as u32,
            x: *x,
            y: *y,
            source: None
        }).collect::<Vec<PixelUpdate>>();
    }

    pub async fn set_pixel(&self, location: Location, color: Color) {
        if location.0 < self.max_height && location.1 < self.max_width && location.0 >= 0 && location.1 >= 0 {
            let mut canvas = self.canvas.write().await;
            println!("Setting location ({}, {}) to color {}", location.0, location.1, color);
            canvas.insert(location, Pixel::from_color(color));
            self.broadcaster.send(PixelUpdate{
                color: color as u32,
                x: location.0,
                y: location.1,
                source: None
            });
        } else {
            println!("Setting pixel failed!")
        }
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<PixelUpdate> {
        self.broadcaster.subscribe()
        /*let (rx, tx) = mpsc::channel(256);
        self.listeners.push(rx);
        tx*/

    }

    pub fn get_metadata(&self) -> CanvasMetadata {
        CanvasMetadata {
            x_size: self.max_width,
            y_size: self.max_height,
            subscriber_count: self.broadcaster.receiver_count() as u64
        }
    }
}