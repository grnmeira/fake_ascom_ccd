#![allow(warnings)]
use ascom_alpaca::api::camera::{CameraState, ImageArray, SensorType};
use ascom_alpaca::api::{Camera, CargoServerInfo, Device};
use ascom_alpaca::{ASCOMError, ASCOMResult, Server};
use async_trait::async_trait;
use log::{debug, error, info, warn};
use std::convert::Infallible;
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

#[derive(Debug)]
struct FakeCameraState {
    camera_state: CameraState,
    last_exposure_length: f64,
    last_exposure_timestamp: f64,
    image_ready: bool,
}

#[derive(Debug)]
struct FakeCamera {
    state: Mutex<FakeCameraState>,
}

impl FakeCamera {
    fn new_idle_camera() -> FakeCamera {
        FakeCamera {
            state: Mutex::new(FakeCameraState {
                camera_state: CameraState::Idle,
                last_exposure_length: 0.0,
                last_exposure_timestamp: 0.0,
                image_ready: false,
            }),
        }
    }
}

#[async_trait]
impl Device for FakeCamera {
    fn static_name(&self) -> &str {
        "Fake CCD Camera"
    }

    fn unique_id(&self) -> &str {
        "f2ac654a-a710-41ab-9484-33f4e73b180b"
    }

    async fn connected(&self) -> ASCOMResult<bool> {
        debug!("Device::connected() = true");
        Ok(true)
    }

    async fn set_connected(&self, connected: bool) -> ASCOMResult<()> {
        debug!("Device::set_connected({:?})", connected);
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
impl Camera for FakeCamera {
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
        Ok(false)
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
        debug!(
            "Camera::start_exposure(duration={}, light={})",
            duration, light
        );
        if let mut state = self.state.lock().await {
            state.image_ready = false;
            state.camera_state = CameraState::Exposing;
            state.last_exposure_length = duration;
            state.last_exposure_timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            Ok(())
        } else {
            // TODO: check the standard what's the correct
            // type of error in these cases.
            ASCOMResult::Err(ASCOMError::VALUE_NOT_SET)
        }
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

    async fn set_num_y(&self, num_y: i32) -> ASCOMResult<()> {
        debug!("Camera::set_num_y({})", num_y);
        Ok(())
    }

    async fn set_num_x(&self, num_x: i32) -> ASCOMResult<()> {
        debug!("Camera::set_num_x({})", num_x);
        Ok(())
    }

    async fn num_x(&self) -> ASCOMResult<i32> {
        Ok(1000)
    }

    async fn num_y(&self) -> ASCOMResult<i32> {
        Ok(1000)
    }

    async fn sensor_type(&self) -> ASCOMResult<SensorType> {
        debug!("Camera::sensor_type() = {:?}", SensorType::Color);
        Ok(SensorType::Color)
    }

    async fn camera_state(&self) -> ASCOMResult<CameraState> {
        if let mut state = self.state.lock().await {
            debug!("Camera::camera_state() = {:?}", state.camera_state);
            Ok(state.camera_state)
        } else {
            // TODO: check the standard what's the correct type
            // to return here.
            ASCOMResult::Err(ASCOMError::VALUE_NOT_SET)
        }
    }

    async fn image_ready(&self) -> ASCOMResult<bool> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let mut state = self.state.lock().await;
        debug!("Camera::image_ready() = {:?}", state.image_ready);
        if state.image_ready {
            return Ok(true);
        }
        let exposure_time = now - state.last_exposure_timestamp;
        state.image_ready = state.camera_state == CameraState::Exposing
            && exposure_time > state.last_exposure_length;
        if state.image_ready {
            state.camera_state = CameraState::Idle;
        }
        Ok(state.image_ready)
    }

    async fn image_array(&self) -> ASCOMResult<ImageArray> {
        debug!("Camera::image_array() = ...");
        let t1 = Instant::now();
        let data = vec![1000; 3000000];
        let t2 = Instant::now();
        let mut arr = ndarray::Array::from_shape_vec((1000, 1000, 3), data).unwrap();
        let t3 = Instant::now();
        debug!("{:?}, {:?}", t2 - t1, t3 - t2);
        ASCOMResult::Ok(arr.into())
    }
}

#[tokio::main]
async fn main() -> eyre::Result<Infallible> {
    env_logger::init();
    // create with the helper macro that populate server information from your own Cargo.toml
    let mut server = Server::new(CargoServerInfo!());

    // By default, the server will listen on dual-stack (IPv4 + IPv6) unspecified address with a randomly assigned port.
    // You can change that by modifying the `listen_addr` field:
    server.listen_addr.set_port(8000);

    // Create and register your device(s).
    server.devices.register(FakeCamera::new_idle_camera());

    // Start the infinite server loop.
    info!("ALPACA server initialized.");
    let r = server.start().await;
    info!("ALPACA server terminated.");
    r
}
