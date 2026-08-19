use crate::sysinfo_proto::sysinfo::SerialDevice;

pub fn collect_serial_devices() -> Vec<SerialDevice> {
    collect_serial_devices_impl()
}

#[cfg(target_os = "linux")]
fn collect_serial_devices_impl() -> Vec<SerialDevice> {
    use tokio_serial::{SerialPortType, available_ports};

    let Ok(ports) = available_ports() else {
        return Vec::new();
    };

    let mut devices: Vec<SerialDevice> = ports
        .into_iter()
        .map(|port_info| {
            let (port_type, vid, pid, serial_number, manufacturer, product) =
                match &port_info.port_type {
                    SerialPortType::UsbPort(usb_info) => (
                        "usb".to_string(),
                        usb_info.vid as u32,
                        usb_info.pid as u32,
                        usb_info.serial_number.clone().unwrap_or_default(),
                        usb_info.manufacturer.clone().unwrap_or_default(),
                        usb_info.product.clone().unwrap_or_default(),
                    ),
                    SerialPortType::PciPort => (
                        "pci".to_string(),
                        0,
                        0,
                        String::new(),
                        String::new(),
                        String::new(),
                    ),
                    SerialPortType::BluetoothPort => (
                        "bluetooth".to_string(),
                        0,
                        0,
                        String::new(),
                        String::new(),
                        String::new(),
                    ),
                    SerialPortType::Unknown => (
                        "unknown".to_string(),
                        0,
                        0,
                        String::new(),
                        String::new(),
                        String::new(),
                    ),
                };

            SerialDevice {
                port_name: port_info.port_name,
                vid,
                pid,
                serial_number,
                manufacturer,
                product,
                port_type,
            }
        })
        .collect();

    devices.sort_by(|a, b| a.port_name.cmp(&b.port_name));
    devices
}

#[cfg(not(target_os = "linux"))]
fn collect_serial_devices_impl() -> Vec<SerialDevice> {
    Vec::new()
}
