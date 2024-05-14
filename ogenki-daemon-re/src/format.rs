use twelite_serial::*;

pub trait FormatExt {
    fn format(&self) -> String;
}

impl FormatExt for StatusNotify {
    fn format(&self) -> String {
        let dbm = self.lqi_dbm();
        let mv = self.power_voltage_millis();
        let open = self.di1_status();
        let changed = self.di1_changed();

        format!("{dbm:.2}dBm {mv}mV is_open: {open} changed: {changed}")
    }
}
