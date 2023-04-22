use std::fmt::{Display, Formatter};

use dashmap::DashMap;
use tokio::sync::broadcast;

use crate::{CanvasMetadata, PixelUpdate};

type Location = (u64, u64);
pub(crate) type Color = u8;

#[derive(Copy, Clone, Debug, Default)]
pub struct Pixel {
    color: Color,
}

impl Pixel {
    fn from_color(color: Color) -> Self {
        Pixel { color }
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
    canvas: DashMap<Location, Pixel>,
    max_width: u64,
    max_height: u64,
    broadcaster: broadcast::Sender<PixelUpdate>, // listeners: Vec<mpsc::Sender<Result<PixelUpdate, Status>>>
}

impl Playground {
    pub fn new(width: u64, height: u64) -> Self {
        Playground {
            canvas: Default::default(),
            max_width: width,
            max_height: height,

            broadcaster: broadcast::channel(256).0, // listeners: vec![]
        }
    }

    pub async fn get_pixel_at_location(&self, location: &Location) -> Pixel {
        match self.canvas.get(location) {
            Some(pixel) => *pixel,
            None => Pixel::default(),
        }
    }

    pub async fn get_pixels_as_updates(&self) -> impl Iterator<Item = PixelUpdate> + '_ {
        self.canvas.iter().map(|r| PixelUpdate {
            color: r.value().color as u32,
            x: r.key().0,
            y: r.key().1,
            source: None,
        })
    }

    pub async fn set_pixel(&self, location: Location, color: Color) {
        if location.0 < self.max_height && location.1 < self.max_width {
            println!(
                "Setting location ({}, {}) to color {}",
                location.0, location.1, color
            );
            self.canvas.insert(location, Pixel::from_color(color));
            self.broadcaster.send(PixelUpdate {
                color: color as u32,
                x: location.0,
                y: location.1,
                source: None,
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
            subscriber_count: self.broadcaster.receiver_count() as u64,
        }
    }
}
