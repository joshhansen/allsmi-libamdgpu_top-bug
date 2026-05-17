use all_smi::AllSmi;
use all_smi::device::reader_factory::get_gpu_readers;
use all_smi::device::readers::amd::AmdGpuReader;

use libamdgpu_top::AMDGPU::DeviceHandle;
use libamdgpu_top::DevicePath;
use libamdgpu_top::LibDrmAmdgpu;

fn main() {
    for i in 0..1024usize {
        println!("{}", i);
        // Triggers the bug
        // AllSmi::new().unwrap();

        // Triggers the bug
        // get_gpu_readers();

        // Triggers the bug
        // AmdGpuReader::default();

        // Triggers the bug
        for p in DevicePath::get_device_path_list().into_iter().take(1) {
            // println!("{:?}", p);
            let handle = p.init().unwrap();
        }

        // Does not trigger the bug
        // DevicePath::get_device_path_list();

        // Does not trigger the bug
        // LibDrmAmdgpu::new();
    }
}
