use smart_house::{SmartDevice, SmartDeviceList};

trait BatchVisit {
    fn visit_all<V: Visitor>(&self, switcher: V);
}

trait Visitor {
    fn execute(&self, target: &mut SmartDevice);
}
impl BatchVisit for SmartDeviceList {
    fn visit_all<V: Visitor>(&self, switcher: V) {
        let device_list = self.clone();
        device_list
            .get_inner_list()
            .iter_mut()
            .for_each(|mut i| i.iter_mut().for_each(|d| switcher.execute(d)));
    }
}
struct SocketSwitcher;

impl Visitor for SocketSwitcher {
    fn execute(&self, device: &mut SmartDevice) {
        if let SmartDevice::Socket(s) = device {
            s.turn_off()
        }
    }
}
#[cfg(test)]
mod test {
    use crate::{BatchVisit, SocketSwitcher};
    use smart_house::{
        DeviceInfo, DeviceInfoProvider, PowerSocket, PowerSocketState, SmartDevice, SmartDeviceList,
    };

    #[test]
    fn batch_turn_off() {
        let devices = sample_list_helper();
        //smart sockets are switched on:
        assert_eq!(
            devices.get_device_info("hall", "sock2").unwrap(),
            DeviceInfo {
                kind: "SmartSocket".into(),
                name: "sock2".into(),
                state: "Powered(220)".into()
            }
        );
        assert_eq!(
            devices.get_device_info("bedroom", "sock1").unwrap(),
            DeviceInfo {
                kind: "SmartSocket".into(),
                name: "sock1".into(),
                state: "Powered(220)".into()
            }
        );

        devices.visit_all(SocketSwitcher);
        //smart sockets are switched off:
        assert_eq!(
            devices.get_device_info("hall", "sock2").unwrap(),
            DeviceInfo {
                kind: "SmartSocket".into(),
                name: "sock2".into(),
                state: "NotPowered".into()
            }
        );
        assert_eq!(
            devices.get_device_info("bedroom", "sock1").unwrap(),
            DeviceInfo {
                kind: "SmartSocket".into(),
                name: "sock1".into(),
                state: "NotPowered".into()
            }
        );
    }

    fn sample_list_helper() -> SmartDeviceList {
        let mut list = SmartDeviceList::new();
        let socket1 = SmartDevice::Socket(PowerSocket {
            name: "sock1".into(),
            state: PowerSocketState::Powered(220),
            description: "".into(),
            power_consumption: 220,
        });
        let socket2 = SmartDevice::Socket(PowerSocket {
            name: "sock2".into(),
            state: PowerSocketState::Powered(220),
            description: "".into(),
            power_consumption: 220,
        });
        list.add_device("bedroom", socket1).unwrap();
        list.add_device("hall", socket2).unwrap();
        list
    }
}
