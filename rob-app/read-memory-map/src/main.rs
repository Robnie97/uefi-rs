#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::boot;
use uefi::mem::memory_map::MemoryMap;


//声明全局分配器
#[global_allocator]
static ALLOCATOR: uefi::allocator::Allocator = uefi::allocator::Allocator;


#[entry]
fn main() -> Status {
    //initialize uefi services
    uefi::helpers::init().unwrap();

    //get memory map
    let memory_map = boot::memory_map(boot::MemoryType::LOADER_DATA)
            .expect("Failed to retrieve UEFI memory map");

    //初始化分配器（传入内存映射的引用
    //在旧版 uefi-rs 中，确实需要手动调用 init(&boot_services) 来初始化分配器。
    // 但在你使用的 0.35 或更新版本中，这个设计已经简化，手动初始化的步骤被取消了。
    //uefi::allocator::init(&memory_map); //新版本拿掉了这个

    // traverse memory map and print each entry
    for desc in memory_map.entries() {
        //print memory start address, size, memory type and attributes
        //size = page_count * 4096 (page size)
        log::info!(
            "start:{:#016x}, size ={:#016x}, type={:?}, attributes={:?}",
            desc.phys_start,
            desc.page_count * 4096,
            desc.ty,
            desc.att
        );
    }

    Status::SUCCESS
}
