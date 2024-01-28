use std::path::{Path, PathBuf};
use sysinfo;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Disk {
    mount_point: PathBuf,
    display_name: String,
}

impl Disk {
    pub fn from_sysinfo_disk(disk: &sysinfo::Disk) -> Self {
        Self {
            mount_point: disk.mount_point().to_path_buf(),
            // TODO: Get display name of the devices.
            // Currently on Linux it returns "/dev/sda1" and on Windows it returns an
            // empty string for the C:\\ drive.
            display_name: disk.name().to_str().unwrap_or_default().to_string(),
        }
    }

    pub fn mount_point(&self) -> &Path {
        &self.mount_point
    }

    pub fn display_name(&self) -> &str {
        &self.display_name
    }
}

#[derive(Default)]
pub struct Disks {
    disks: Vec<Disk>,
}

impl Disks {
    pub fn new_with_refreshed_list() -> Self {
        let disks = sysinfo::Disks::new_with_refreshed_list();

        let mut result: Vec<Disk> = Vec::new();
        for disk in disks.iter() {
            result.push(Disk::from_sysinfo_disk(disk));
        }

        Self { disks: result }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Disk> {
        self.disks.iter()
    }
}
