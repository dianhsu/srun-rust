use uuid::Uuid;
pub fn rand_str() -> String {
    let id = Uuid::new_v4();
    id.to_string()
}

pub fn get_ip_from_interface(interface: &str) -> String {
    if interface.len() == 0 {
        match default_net::get_default_interface() {
            Ok(default_interface) => {
                format!("{:?}", default_interface.ipv4.first().unwrap().addr)
            }
            Err(_) => {
                log::error!("can't get default interface");
                "".to_string()
            }
        }
    } else {
        for interface_item in default_net::get_interfaces() {
            let interface_name = interface_item.name;
            if let Some(mac_addr) = interface_item.mac_addr {
                log::info!("interface: {} mac address: {}", interface_name, mac_addr);
                if mac_addr.address().eq(&interface) || interface_name.eq(&interface) {
                    return format!("{:?}", interface_item.ipv4.first().unwrap().addr);
                }
            } else {
                log::info!("interface: {} failed to get mac address", interface_name);
                if interface_name.eq(&interface) {
                    return format!("{:?}", interface_item.ipv4.first().unwrap().addr);
                }
            }
        }
        log::error!("can't find interface: {}", interface);
        return "".to_string();
    }
}
