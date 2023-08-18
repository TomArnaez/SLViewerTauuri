use autocxx::prelude::*;
use cxx::SharedPtr;
use image::buffer;
use imageproc::definitions::Image;
use imageproc::filter::median_filter;

use std::mem;
use std::thread;
use std::time::Duration;

use image::{ImageBuffer, Luma};

use crate::cpp::ffi::SpectrumLogic::SLError;
use crate::statistics;

include_cpp! {
    #include "SLDevice.h"
    safety!(unsafe_ffi)
    generate!("SpectrumLogic::SLDevice")
    generate!("SpectrumLogic::DeviceInterface")
}

pub struct Detector {
    device_instance: UniquePtr<ffi::SpectrumLogic::SLDevice>,
}

trait Capture {
    fn new(device_instance: UniquePtr<ffi::SpectrumLogic::SLDevice>) -> Self;

    fn start(&self);
    fn get_captures(&self) -> &Vec<ImageBuffer<Luma<u16>, Vec<u16>>>;
}

struct SignalAccumulationCapture {
    device_instance: UniquePtr<ffi::SpectrumLogic::SLDevice>,
    captures: Vec<ImageBuffer<Luma<u16>, Vec<u16>>>,
}
struct SmartCapture {
    device_instance: UniquePtr<ffi::SpectrumLogic::SLDevice>,
    captures: Vec<ImageBuffer<Luma<u16>, Vec<u16>>>,
    exp_times: Vec<u32>,
    frames_per_capture: u32,
}

impl Capture for SmartCapture {
    fn new(device_instance: UniquePtr<ffi::SpectrumLogic::SLDevice>) -> Self {
        SmartCapture {
            device_instance,
            captures: Vec::new(),
            exp_times: vec![200, 500, 1000, 2000, 10000, 15000, 30000],
            frames_per_capture: 10,
        }
    }

    fn start(&self) {
        let mut SNRs: Vec<f64> = Vec::new();
        println!("Starting smart capture");
        let window_size = 3;

        const width: usize = 1031;
        const height: usize = 1536;
        const imageSize: usize = 1031 * 1536;

        let mut SNRs: Vec<u64> = Vec::new();

        for buffer in 0..self.exp_times.len() {
            let mut capture_image: ImageBuffer<Luma<u16>, Vec<u16>> =
                ImageBuffer::new(width as u32, height as u32);
        }
    }
    fn get_captures(&self) -> &Vec<ImageBuffer<Luma<u16>, Vec<u16>>> {
        return &self.captures;
    }
}

impl Detector {
    pub fn new() -> Self {
        Detector {
            device_instance: ffi::SpectrumLogic::SLDevice::new(
                ffi::SpectrumLogic::DeviceInterface::USB,
                autocxx::c_int(2),
                "",
                "",
                "",
            )
            .within_unique_ptr(),
        }
    }

    pub fn connect(&mut self) {
        self.device_instance
            .pin_mut()
            .OpenCamera(autocxx::c_int(100));
    }

    pub fn set_exposure_time(&mut self, exp_time: u32) {
        self.device_instance
            .pin_mut()
            .SetExposureTime(autocxx::c_int(exp_time as i32));
    }

    pub fn go_live(&mut self) {
        self.device_instance.pin_mut().GoLive();
    }

    pub fn start_stream(&mut self, streamTime: u32, exposureTime: u32) {
        self.connect();
        self.set_exposure_time(500);

        self.device_instance
            .pin_mut()
            .StartStream(autocxx::c_int(5 as i32));

        const imageSize: usize = 1031 * 1536;
        let mut rust_vec: Vec<autocxx::c_ushort> = vec![autocxx::c_ushort(0); imageSize];

        unsafe {
            self.device_instance
                .pin_mut()
                .ReadFrame(rust_vec.as_mut_ptr(), false);
        }

        let u16_ptr: *mut u16 = rust_vec.as_mut_ptr() as *mut u16;
        let u16_vec: Vec<u16> = unsafe {
            Vec::from_raw_parts(
                u16_ptr,
                imageSize * std::mem::size_of::<c_ushort>(),
                imageSize * std::mem::size_of::<c_ushort>(),
            )
        };

        for &value in &u16_vec {
            if value > 0 {
                println!("Value above 0: {}", value);
            }
        }

        self.device_instance.pin_mut().CloseCamera();
    }

    pub fn signal_accumulation(&mut self, exp_time: u32, frame_count: i32) {
        const width: usize = 1031;
        const height: usize = 1536;
        const imageSize: usize = 1031 * 1536;

        println!("Running signal accumulation");

        self.device_instance
            .pin_mut()
            .OpenCamera(autocxx::c_int(100));

        self.device_instance.pin_mut().SetTestMode(false);
        self.set_exposure_time(exp_time);

        self.device_instance
            .pin_mut()
            .SetExposureMode(ffi::SpectrumLogic::ExposureModes::seq_mode);

        self.device_instance
            .pin_mut()
            .SetNumberOfFrames(autocxx::c_int(frame_count));

        if (self.device_instance.pin_mut().GoLive()
            != ffi::SpectrumLogic::SLError::SL_ERROR_SUCCESS)
        {
            println!("Failed to Go Live");
            return;
        }

        if (self.device_instance.pin_mut().SoftwareTrigger()
            != ffi::SpectrumLogic::SLError::SL_ERROR_SUCCESS)
        {
            println!("Failed to send software trigger");
            return;
        }

        let mut signal_images: Vec<ImageBuffer<Luma<u16>, Vec<u16>>> = Vec::new();
        let mut rust_vec: Vec<c_ushort> = vec![c_ushort(0); imageSize];
        println!("{frame_count}");

        let duration = Duration::from_millis((exp_time as u64 + 50) * frame_count as u64);
        thread::sleep(duration);

        for buf_num in 0..=frame_count {
            unsafe {
                if (self.device_instance.pin_mut().ReadBuffer1(
                    rust_vec.as_mut_ptr(),
                    autocxx::c_int(buf_num),
                    autocxx::c_int(1000),
                ) != SLError::SL_ERROR_SUCCESS)
                {
                    println!("Failed to read frame");
                    break;
                }

                // Convert the slice to a Vec<u16>
                let u16_vec: Vec<u16> =
                    std::slice::from_raw_parts(rust_vec.as_ptr() as *const u16, width * height)
                        .to_vec();

                let mut current_image = ImageBuffer::<Luma<u16>, Vec<u16>>::from_vec(
                    width as u32,
                    height as u32,
                    u16_vec,
                )
                .expect("Failed to create ImageBuffer");

                if let Some(prev_image) = signal_images.last() {
                    for (prev_image_pixel, current_image_pixel) in
                        prev_image.pixels().zip(current_image.pixels_mut())
                    {
                        current_image_pixel[0] =
                            current_image_pixel[0].saturating_add(prev_image_pixel[0])
                    }
                }

                current_image
                    .save(format!("target/signal{buf_num}.png"))
                    .expect("Failed to save image");

                signal_images.push(current_image);
            }
        }
    }

    pub fn multi_exposure(&mut self, exp_times: Vec<u32>) {
        const width: usize = 1031;
        const height: usize = 1536;
        const imageSize: usize = 1031 * 1536;

        let mut SNRs: Vec<u64> = Vec::new();

        self.device_instance
            .pin_mut()
            .OpenCamera(autocxx::c_int(100));

        self.device_instance
            .pin_mut()
            .SetExposureMode(ffi::SpectrumLogic::ExposureModes::trig_mode);

        let mut exp_images: Vec<ImageBuffer<Luma<u16>, Vec<u16>>> = Vec::new();

        for exp_time in exp_times {
            self.device_instance
                .pin_mut()
                .SetExposureTime(autocxx::c_int(exp_time as i32));

            self.device_instance.pin_mut().SoftwareTrigger();
            let duration = Duration::from_millis(exp_time as u64);
            thread::sleep(duration);
        }
    }
}
