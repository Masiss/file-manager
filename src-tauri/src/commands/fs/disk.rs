use anyhow::Result;
use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};

#[derive(Serialize, Deserialize, Debug)]
pub struct Disk {
    pub name: String,
    available_space: u64,
    pub mount_point: String,
    is_removable: bool,
    total_space: u64,
    kind: String,
    path: String,
}
pub fn get_disk() -> Result<Vec<Disk>> {
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
            total_space: disk.total_space(),
            path: String::from(disk.mount_point().to_string_lossy()),
        });
    }
    Ok(list_disk)
}
#[tauri::command]
pub fn load_disk() -> Result<Vec<Disk>, String> {
    let list_disk = get_disk().map_err(|e| format!("Error on getting disk : {}", e))?;
    Ok(list_disk)
}
