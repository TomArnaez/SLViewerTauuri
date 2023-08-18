use autocxx::prelude::*;
include_cpp! {
    #include "SLDevice.h"
    safety!(unsafe_ffi)
    generate!("SpectrumLogic::SLDevice")
    generate!("SpectrumLogic::DeviceInterface")
}

struct Detector {
    device_instance: UniquePtr<ffi::SpectrumLogic::SLDevice>,
}

impl Detector {
    fn new() -> Self {
        Detector {
            device_instance: ffi::SpectrumLogic::SLDevice::new(
                ffi::SpectrumLogic::DeviceInterface::PLEORA,
                autocxx::c_int(2),
                "",
                "",
                "",
            )
            .within_unique_ptr(),
        }
    }

    fn connect(&mut self) {
        self.device_instance
            .pin_mut()
            .OpenCamera(autocxx::c_int(100));
    }

    fn set_exposure_time(&mut self, exp_time: i32) {
        self.device_instance
            .pin_mut()
            .SetExposureTime(autocxx::c_int(exp_time));
    }

    fn go_live(&mut self) {
        self.device_instance.pin_mut().GoLive();
    }

    fn start_stream(&mut self) {}
}

pub fn test() {
    let interface = ffi::SpectrumLogic::DeviceInterface::PLEORA;
    println!("{}", interface as u16);
}
