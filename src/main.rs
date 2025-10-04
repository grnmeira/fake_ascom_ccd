use ascom_alpaca::ASCOMResult;
use ascom_alpaca::Server;
use ascom_alpaca::api::CargoServerInfo;
use ascom_alpaca::api::camera::CameraState;
use ascom_alpaca::api::camera::SensorType;
use ascom_alpaca::api::{Camera, Device};
use async_trait::async_trait;
use std::convert::Infallible;

#[derive(Debug)]
struct MyCamera {}

#[async_trait]
impl Device for MyCamera {
    fn static_name(&self) -> &str {
        "Fake CCD Camera"
    }

    fn unique_id(&self) -> &str {
        "f2ac654a-a710-41ab-9484-33f4e73b180b"
    }

    async fn connected(&self) -> ASCOMResult<bool> {
        Ok(true)
    }

    async fn set_connected(&self, connected: bool) -> ASCOMResult<()> {
        Ok(())
    }

    async fn description(&self) -> ASCOMResult<String> {
        Ok("fake CCD camera".to_string())
    }

    async fn driver_info(&self) -> ASCOMResult<String> {
        Ok("".to_string())
    }

    async fn driver_version(&self) -> ASCOMResult<String> {
        Ok("".to_string())
    }
}

#[async_trait]
impl Camera for MyCamera {
    async fn bayer_offset_x(&self) -> ASCOMResult<i32> {
        Ok(0)
    }

    async fn bayer_offset_y(&self) -> ASCOMResult<i32> {
        Ok(0)
    }

    async fn exposure_max(&self) -> ASCOMResult<f64> {
        Ok(120.0)
    }

    async fn exposure_min(&self) -> ASCOMResult<f64> {
        Ok(0.000001)
    }

    async fn exposure_resolution(&self) -> ASCOMResult<f64> {
        Ok(0.0001)
    }

    async fn has_shutter(&self) -> ASCOMResult<bool> {
        Ok(true)
    }

    async fn max_adu(&self) -> ASCOMResult<i32> {
        Ok(65535)
    }

    async fn pixel_size_x(&self) -> ASCOMResult<f64> {
        Ok(0.1)
    }

    async fn pixel_size_y(&self) -> ASCOMResult<f64> {
        Ok(0.1)
    }

    async fn start_x(&self) -> ASCOMResult<i32> {
        Ok(0)
    }

    async fn set_start_x(&self, start_x: i32) -> ASCOMResult<()> {
        Ok(())
    }

    async fn start_y(&self) -> ASCOMResult<i32> {
        Ok(0)
    }

    async fn set_start_y(&self, start_x: i32) -> ASCOMResult<()> {
        Ok(())
    }

    async fn start_exposure(&self, duration: f64, light: bool) -> ASCOMResult<()> {
        Ok(())
    }

    async fn bin_x(&self) -> ASCOMResult<i32> {
        Ok(1)
    }

    async fn bin_y(&self) -> ASCOMResult<i32> {
        Ok(1)
    }

    async fn set_bin_x(&self, bin_x: i32) -> ASCOMResult<()> {
        Ok(())
    }

    async fn set_bin_y(&self, bin_x: i32) -> ASCOMResult<()> {
        Ok(())
    }

    async fn camera_xsize(&self) -> ASCOMResult<i32> {
        Ok(1000)
    }

    async fn camera_ysize(&self) -> ASCOMResult<i32> {
        Ok(1000)
    }

    async fn set_num_y(&self, num_x: i32) -> ASCOMResult<()> {
        Ok(())
    }

    async fn set_num_x(&self, num_x: i32) -> ASCOMResult<()> {
        Ok(())
    }

    async fn num_x(&self) -> ASCOMResult<i32> {
        Ok(1000)
    }

    async fn num_y(&self) -> ASCOMResult<i32> {
        Ok(1000)
    }

    async fn sensor_type(&self) -> ASCOMResult<SensorType> {
        Ok(SensorType::RGGB)
    }

    async fn camera_state(&self) -> ASCOMResult<CameraState> {
        Ok(CameraState::Exposing)
    }

    async fn image_ready(&self) -> ASCOMResult<bool> {
        Ok(false)
    }
}

#[tokio::main]
async fn main() -> eyre::Result<Infallible> {
    // create with the helper macro that populate server information from your own Cargo.toml
    let mut server = Server::new(CargoServerInfo!());

    // By default, the server will listen on dual-stack (IPv4 + IPv6) unspecified address with a randomly assigned port.
    // You can change that by modifying the `listen_addr` field:
    server.listen_addr.set_port(8000);

    // Create and register your device(s).
    server.devices.register(MyCamera { /* ... */ });

    // Start the infinite server loop.
    server.start().await
}
