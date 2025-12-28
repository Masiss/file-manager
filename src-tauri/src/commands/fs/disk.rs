use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};
#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    name: String,
    available_space: u64,
    mount_point: String,
    is_removable: bool,
    usage: u64,
    total_space: u64,
    kind: String,
}
#[tauri::command]
pub fn load_disk() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();
    let disks = Disks::new_with_refreshed_list();
    let mut list_disk: Vec<Disk> = Vec::new();
    for disk in &disks {
        list_disk.push(Disk {
            name: String::from(disk.name().to_string_lossy()),
            kind: disk.kind().to_string(),
            available_space: disk.available_space(),
            mount_point: String::from(disk.mount_point().to_string_lossy()),
            is_removable: disk.is_removable(),
            usage: disk.usage().total_read_bytes,
            total_space: disk.total_space(),
        });
        // println!("==============");
        // println!("Disk: {}", disk.name().to_string_lossy());
        // println!("Type: {:?}", disk.kind());
        // println!("Total space: {} GB", disk.total_space() / 1_000_000_000);
        // println!(
        //     "Available space: {} GB",
        //     disk.available_space() / 1_000_000_000
        // );
        // println!("Mount point: {:?}", disk.mount_point());
        // println!("File system: {:?}", disk.file_system());
    }

    serde_json::to_string(&list_disk).unwrap()
}
