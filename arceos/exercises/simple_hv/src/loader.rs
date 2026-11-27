use std::io::{self, Read};
use std::fs::File;
use axhal::paging::MappingFlags;
use axhal::mem::PAGE_SIZE_4K;
use axmm::AddrSpace;
use crate::VM_ENTRY;

pub fn load_vm_image(fname: &str, uspace: &mut AddrSpace) -> io::Result<()> {
    let mut buf = [0u8; PAGE_SIZE_4K];
    let n = load_file(fname, &mut buf)?;

    uspace.map_alloc(VM_ENTRY.into(), PAGE_SIZE_4K, MappingFlags::READ|MappingFlags::WRITE|MappingFlags::EXECUTE|MappingFlags::USER, true).unwrap();
    uspace.write(VM_ENTRY.into(), &buf[..n]).unwrap();

    uspace
        .map_alloc(
            0.into(),
            PAGE_SIZE_4K,
            MappingFlags::READ | MappingFlags::WRITE | MappingFlags::USER,
            true,
        )
        .unwrap();
    let mut boot_data = [0u8; PAGE_SIZE_4K];
    boot_data[64..72].copy_from_slice(&0x6688usize.to_le_bytes());
    uspace.write(0.into(), &boot_data).unwrap();

    Ok(())
}

fn load_file(fname: &str, buf: &mut [u8]) -> io::Result<usize> {
    ax_println!("app: {}", fname);
    let mut file = File::open(fname)?;
    let n = file.read(buf)?;
    Ok(n)
}
