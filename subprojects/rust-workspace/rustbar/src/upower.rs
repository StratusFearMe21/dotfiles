// This code was autogenerated with `dbus-codegen-rust -s -g -m None -d org.freedesktop.UPower -p /org/freedesktop/UPower`, see https://github.com/diwic/dbus-rs
use dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;
use num_enum::FromPrimitive;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, FromPrimitive)]
#[repr(u32)]
pub enum BatteryState {
    #[default]
    Unknown = 0,
    Charging = 1,
    Discharging = 2,
    Empty = 3,
    FullyCharged = 4,
    PendingCharge = 5,
    PendingDischarge = 6,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, FromPrimitive)]
#[repr(u32)]
pub enum WarningLevel {
    #[default]
    Unknown = 0,
    None = 1,
    Discharging = 2,
    Low = 3,
    Critical = 4,
    Action = 5,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default, FromPrimitive)]
#[repr(u32)]
pub enum BatteryType {
    #[default]
    Unknown = 0,
    LinePower = 1,
    Battery = 2,
    Ups = 3,
    Monitor = 4,
    Mouse = 5,
    Keyboard = 6,
    Pda = 7,
    Phone = 8,
    MediaPlayer = 9,
    Tablet = 10,
    Computer = 11,
    GamingInput = 12,
    Pen = 13,
    Touchpad = 14,
    Modem = 15,
    Network = 16,
    Headset = 17,
    Speakers = 18,
    Headphones = 19,
    Video = 20,
    OtherAudio = 21,
    RemoteControl = 22,
    Printer = 23,
    Scanner = 24,
    Camera = 25,
    Wearable = 26,
    Toy = 27,
    BluetoothGeneric = 28,
    Last = 29,
}

pub trait OrgFreedesktopDBusProperties {
    fn get<R0: for<'b> arg::Get<'b> + 'static>(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> Result<R0, dbus::Error>;
    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error>;
    fn set<I2: arg::Arg + arg::Append>(
        &self,
        interface_name: &str,
        property_name: &str,
        value: I2,
    ) -> Result<(), dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopDBusPropertiesPropertiesChanged {
    pub interface_name: String,
    pub changed_properties: arg::PropMap,
    pub invalidated_properties: Vec<String>,
}

impl arg::AppendAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.interface_name, i);
        arg::RefArg::append(&self.changed_properties, i);
        arg::RefArg::append(&self.invalidated_properties, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusPropertiesPropertiesChanged {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusPropertiesPropertiesChanged {
            interface_name: i.read()?,
            changed_properties: i.read()?,
            invalidated_properties: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusPropertiesPropertiesChanged {
    const NAME: &'static str = "PropertiesChanged";
    const INTERFACE: &'static str = "org.freedesktop.DBus.Properties";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopDBusProperties
    for blocking::Proxy<'a, C>
{
    fn get<R0: for<'b> arg::Get<'b> + 'static>(
        &self,
        interface_name: &str,
        property_name: &str,
    ) -> Result<R0, dbus::Error> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "Get",
            (interface_name, property_name),
        )
        .and_then(|r: (arg::Variant<R0>,)| Ok((r.0).0))
    }

    fn get_all(&self, interface_name: &str) -> Result<arg::PropMap, dbus::Error> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "GetAll",
            (interface_name,),
        )
        .and_then(|r: (arg::PropMap,)| Ok(r.0))
    }

    fn set<I2: arg::Arg + arg::Append>(
        &self,
        interface_name: &str,
        property_name: &str,
        value: I2,
    ) -> Result<(), dbus::Error> {
        self.method_call(
            "org.freedesktop.DBus.Properties",
            "Set",
            (interface_name, property_name, arg::Variant(value)),
        )
    }
}

pub trait OrgFreedesktopDBusIntrospectable {
    fn introspect(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>>
    OrgFreedesktopDBusIntrospectable for blocking::Proxy<'a, C>
{
    fn introspect(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Introspectable", "Introspect", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait OrgFreedesktopDBusPeer {
    fn ping(&self) -> Result<(), dbus::Error>;
    fn get_machine_id(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopDBusPeer
    for blocking::Proxy<'a, C>
{
    fn ping(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "Ping", ())
    }

    fn get_machine_id(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.DBus.Peer", "GetMachineId", ())
            .and_then(|r: (String,)| Ok(r.0))
    }
}

pub trait OrgFreedesktopUPower {
    fn enumerate_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error>;
    fn get_display_device(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn get_critical_action(&self) -> Result<String, dbus::Error>;
    fn daemon_version(&self) -> Result<String, dbus::Error>;
    fn on_battery(&self) -> Result<bool, dbus::Error>;
    fn lid_is_closed(&self) -> Result<bool, dbus::Error>;
    fn lid_is_present(&self) -> Result<bool, dbus::Error>;
}

#[derive(Debug)]
pub struct OrgFreedesktopUPowerDeviceAdded {
    pub device: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopUPowerDeviceAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.device, i);
    }
}

impl arg::ReadAll for OrgFreedesktopUPowerDeviceAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopUPowerDeviceAdded { device: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopUPowerDeviceAdded {
    const NAME: &'static str = "DeviceAdded";
    const INTERFACE: &'static str = "org.freedesktop.UPower";
}

#[derive(Debug)]
pub struct OrgFreedesktopUPowerDeviceRemoved {
    pub device: dbus::Path<'static>,
}

impl arg::AppendAll for OrgFreedesktopUPowerDeviceRemoved {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.device, i);
    }
}

impl arg::ReadAll for OrgFreedesktopUPowerDeviceRemoved {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopUPowerDeviceRemoved { device: i.read()? })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopUPowerDeviceRemoved {
    const NAME: &'static str = "DeviceRemoved";
    const INTERFACE: &'static str = "org.freedesktop.UPower";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopUPower
    for blocking::Proxy<'a, C>
{
    fn enumerate_devices(&self) -> Result<Vec<dbus::Path<'static>>, dbus::Error> {
        self.method_call("org.freedesktop.UPower", "EnumerateDevices", ())
            .and_then(|r: (Vec<dbus::Path<'static>>,)| Ok(r.0))
    }

    fn get_display_device(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        self.method_call("org.freedesktop.UPower", "GetDisplayDevice", ())
            .and_then(|r: (dbus::Path<'static>,)| Ok(r.0))
    }

    fn get_critical_action(&self) -> Result<String, dbus::Error> {
        self.method_call("org.freedesktop.UPower", "GetCriticalAction", ())
            .and_then(|r: (String,)| Ok(r.0))
    }

    fn daemon_version(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower",
            "DaemonVersion",
        )
    }

    fn on_battery(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower",
            "OnBattery",
        )
    }

    fn lid_is_closed(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower",
            "LidIsClosed",
        )
    }

    fn lid_is_present(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower",
            "LidIsPresent",
        )
    }
}
// This code was autogenerated with `dbus-codegen-rust -s -g -m None -d org.freedesktop.UPower -p /org/freedesktop/UPower/devices/DisplayDevice`, see https://github.com/diwic/dbus-rs
pub trait OrgFreedesktopUPowerDevice {
    fn refresh(&self) -> Result<(), dbus::Error>;
    fn get_history(
        &self,
        type_: &str,
        timespan: u32,
        resolution: u32,
    ) -> Result<Vec<(u32, f64, u32)>, dbus::Error>;
    fn get_statistics(&self, type_: &str) -> Result<Vec<(f64, f64)>, dbus::Error>;
    fn native_path(&self) -> Result<String, dbus::Error>;
    fn vendor(&self) -> Result<String, dbus::Error>;
    fn model(&self) -> Result<String, dbus::Error>;
    fn serial(&self) -> Result<String, dbus::Error>;
    fn update_time(&self) -> Result<u64, dbus::Error>;
    fn type_(&self) -> Result<u32, dbus::Error>;
    fn power_supply(&self) -> Result<bool, dbus::Error>;
    fn has_history(&self) -> Result<bool, dbus::Error>;
    fn has_statistics(&self) -> Result<bool, dbus::Error>;
    fn online(&self) -> Result<bool, dbus::Error>;
    fn energy(&self) -> Result<f64, dbus::Error>;
    fn energy_empty(&self) -> Result<f64, dbus::Error>;
    fn energy_full(&self) -> Result<f64, dbus::Error>;
    fn energy_full_design(&self) -> Result<f64, dbus::Error>;
    fn energy_rate(&self) -> Result<f64, dbus::Error>;
    fn voltage(&self) -> Result<f64, dbus::Error>;
    fn charge_cycles(&self) -> Result<i32, dbus::Error>;
    fn luminosity(&self) -> Result<f64, dbus::Error>;
    fn time_to_empty(&self) -> Result<i64, dbus::Error>;
    fn time_to_full(&self) -> Result<i64, dbus::Error>;
    fn percentage(&self) -> Result<f64, dbus::Error>;
    fn temperature(&self) -> Result<f64, dbus::Error>;
    fn is_present(&self) -> Result<bool, dbus::Error>;
    fn state(&self) -> Result<u32, dbus::Error>;
    fn is_rechargeable(&self) -> Result<bool, dbus::Error>;
    fn capacity(&self) -> Result<f64, dbus::Error>;
    fn technology(&self) -> Result<u32, dbus::Error>;
    fn warning_level(&self) -> Result<u32, dbus::Error>;
    fn battery_level(&self) -> Result<u32, dbus::Error>;
    fn icon_name(&self) -> Result<String, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target = T>> OrgFreedesktopUPowerDevice
    for blocking::Proxy<'a, C>
{
    fn refresh(&self) -> Result<(), dbus::Error> {
        self.method_call("org.freedesktop.UPower.Device", "Refresh", ())
    }

    fn get_history(
        &self,
        type_: &str,
        timespan: u32,
        resolution: u32,
    ) -> Result<Vec<(u32, f64, u32)>, dbus::Error> {
        self.method_call(
            "org.freedesktop.UPower.Device",
            "GetHistory",
            (type_, timespan, resolution),
        )
        .and_then(|r: (Vec<(u32, f64, u32)>,)| Ok(r.0))
    }

    fn get_statistics(&self, type_: &str) -> Result<Vec<(f64, f64)>, dbus::Error> {
        self.method_call("org.freedesktop.UPower.Device", "GetStatistics", (type_,))
            .and_then(|r: (Vec<(f64, f64)>,)| Ok(r.0))
    }

    fn native_path(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "NativePath",
        )
    }

    fn vendor(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Vendor",
        )
    }

    fn model(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Model",
        )
    }

    fn serial(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Serial",
        )
    }

    fn update_time(&self) -> Result<u64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "UpdateTime",
        )
    }

    fn type_(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Type",
        )
    }

    fn power_supply(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "PowerSupply",
        )
    }

    fn has_history(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "HasHistory",
        )
    }

    fn has_statistics(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "HasStatistics",
        )
    }

    fn online(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Online",
        )
    }

    fn energy(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Energy",
        )
    }

    fn energy_empty(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "EnergyEmpty",
        )
    }

    fn energy_full(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "EnergyFull",
        )
    }

    fn energy_full_design(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "EnergyFullDesign",
        )
    }

    fn energy_rate(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "EnergyRate",
        )
    }

    fn voltage(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Voltage",
        )
    }

    fn charge_cycles(&self) -> Result<i32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "ChargeCycles",
        )
    }

    fn luminosity(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Luminosity",
        )
    }

    fn time_to_empty(&self) -> Result<i64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "TimeToEmpty",
        )
    }

    fn time_to_full(&self) -> Result<i64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "TimeToFull",
        )
    }

    fn percentage(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Percentage",
        )
    }

    fn temperature(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Temperature",
        )
    }

    fn is_present(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "IsPresent",
        )
    }

    fn state(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "State",
        )
    }

    fn is_rechargeable(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "IsRechargeable",
        )
    }

    fn capacity(&self) -> Result<f64, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Capacity",
        )
    }

    fn technology(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "Technology",
        )
    }

    fn warning_level(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "WarningLevel",
        )
    }

    fn battery_level(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "BatteryLevel",
        )
    }

    fn icon_name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(
            &self,
            "org.freedesktop.UPower.Device",
            "IconName",
        )
    }
}
