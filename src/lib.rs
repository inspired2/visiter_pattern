use smart_house::{SmartDeviceList, DeviceInfoProvider, Device};

trait BatchSwitch {
    fn turn_off<T: Device>(&mut self, switcher: impl Switcher<Target=T>) -> ();
}

trait Switcher {
    type Target;
}
impl BatchSwitch for SmartDeviceList {
    fn turn_off<T: Device>(&mut self, switcher: impl Switcher<Target=T>) -> () {
        let device_list = self.clone();
    }
}