#![no_main]
#![no_std]

use uefi::boot::{self, SearchType};
use uefi::print;
use uefi::println;
use uefi::proto::console::text::Key::{Printable, Special};
use uefi::proto::console::text::ScanCode;
use uefi::prelude::*;
use uefi::proto::device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly};
use uefi::proto::loaded_image::LoadedImage;
use uefi::{Identify, Result};

fn print_image_path() -> Result {
    let loaded_image =
        boot::open_protocol_exclusive::<LoadedImage>(boot::image_handle())?;

    let device_path_to_text_handle = *boot::locate_handle_buffer(SearchType::ByProtocol(&DevicePathToText::GUID),)?
    .first()
    .expect("DevicePathToText eksik");

    let device_path_to_text = boot::open_protocol_exclusive::<DevicePathToText>(
        device_path_to_text_handle,
    )?;

    let image_device_path =
        loaded_image.file_path().expect("Dosya yolu belirtilmemis");
    let image_device_path_text = device_path_to_text
        .convert_device_path_to_text(
	    image_device_path,
	    DisplayOnly(true),
            AllowShortcuts(false),
        )
    .expect("convert_device_path_to_text basarisiz");

    print!("Bu dosyadan önyükleniliyor: {}", &*image_device_path_text);
    Ok(())


}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    print_image_path().unwrap();
    print!("
    OVEFI0.4               
                           
    _______________________
    |                     |
    |   / /O ___ ___ ___  |
    | _/_/_  |   |   | |  |
    |_/_/_   |MM |-- | |  |
    |/ /   O |__ |   |_|  |
    |                     |
    |                     |
    |                     |
    |                     |
    |_____________________|
                           
    Merhaba bu UEFI uygulamasi OSVidYapan (eski adiyla WinuxWidYapan)
    tarafindan yazilmistir OVEFI'ya hosgeldiniz ;)
    Cikmak icin Delete tusuna basin
    EFI#:");
    uefi_input2::with_stdin(|input| {
	loop {
	    let Some(_event) = input.wait_for_key_event() else { continue };
	    if let Some(data) = input.read_key_stroke_ex() {
		if data.shift() { println!("Shift tutuluyor") }
		match data.key {
		Printable(c) if u16::from(c) == 0x0D => print!("\r\n"),
		    Printable(c) => print!("{}", c),
		Special(code) if code == ScanCode::DELETE => {
		    println!("Cikiliyor");
		    return Ok(())
		},
		_ => {},
	    }
	}
    }
    }).unwrap();
    uefi::Status(0)
}