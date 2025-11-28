#![deny(clippy::all)]

use napi_derive::napi;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

use std::vec::Vec;

#[napi(constructor)]
#[derive(Debug)]
pub struct NetInterface {
  pub name: String,
  pub addr: Vec<String>,
  pub mac_addr: Option<String>,
  pub index: u32,
  pub is_up: bool,
}

/// Filter Ethernet interfaces.
#[napi]
pub const IFF_ETH: i32 = network_interface::IFF_ETH;

// Filter Wireless interfaces sometimes it sames as Etherent interfaces.
#[napi]
pub const IFF_WIRELESS: i32 = network_interface::IFF_WIRELESS;

/// Filter out VPN interfaces. Note! This is only a hypothesis.
#[napi]
pub const IFF_VPN: i32 = network_interface::IFF_VPN;

/// Filter out TUN interfaces. Note! This is only a hypothesis.
#[napi]
pub const IFF_TUNN: i32 = network_interface::IFF_TUN;

///Filter out LOOPBACK interfaces.
#[napi]
pub const IFF_LOOPBACK: i32 = network_interface::IFF_LOOPBACK;

///Filter Running interfaces
#[napi]
pub const IFF_RUNNING: i32 = network_interface::IFF_RUNNING;

///
/// ```
/// let network_interface = network_interface();
///  
/// ```
///
#[napi]
pub fn interfaces(filter: i32) -> Option<Vec<NetInterface>> {
  let interfaces = match NetworkInterface::show() {
    Ok(ifts) => {
      let mut interfaces = Vec::new();
      for ift in NetworkInterface::filter(ifts, filter) {
        let is_up = ift.is_up();
        interfaces.push(NetInterface {
          name: ift.name,
          addr: ift
            .addr
            .iter()
            .map(|addr| format!("{}", addr.ip()))
            .collect(),
          mac_addr: ift.mac_addr,
          index: ift.index,
          is_up,
        });
      }
      interfaces
    }
    Err(_) => return None,
  };
  Some(interfaces)
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_network_interface() {
    for itf in interfaces(IFF_ETH).unwrap() {
      println!(
        "===={}: {:?} - {:?} - {}",
        itf.index, itf.name, itf.addr, itf.is_up
      );
    }
  }
}
