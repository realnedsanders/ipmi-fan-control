use std::process::Command;

use anyhow::{Error, Ok};
use regex::Regex;

pub(crate) trait Ipmi {
    fn get_info_fan_temp(&self) -> Result<String, Error>;
    fn get_cpu_temperature(&self) -> Result<u16, Error>;
    fn set_fan_speed(&self, fans: u16, speed: u16) -> Result<(), Error>;
}

pub(crate) trait Executer {
    fn get_info_fan_temp(&self) -> Result<String, Error>;
    fn get_cpu_temperature(&self) -> Result<String, Error>;
    fn set_fan_speed(&self, fans: u16, speed: u16) -> Result<(), Error>;

    fn execute(&self, program: &str, args: Vec<&str>) -> Result<String, Error> {
        let output = Command::new(program).args(args).output()?;
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            Ok(stdout.to_string())
        } else {
            Err(anyhow!(
                "status:{}, stderr: {}",
                output.status.code().unwrap(),
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }
}

pub(crate) struct Cmd {}

impl Cmd {
    pub fn new() -> Self {
        Self {}
    }
}

impl Executer for Cmd {
    fn get_info_fan_temp(&self) -> Result<String, Error> {
        self.execute("ipmitool", vec!["sdr", "list", "full"])
    }

    fn get_cpu_temperature(&self) -> Result<String, Error> {
        self.execute("ipmitool", vec!["sdr", "type", "Temperature"])
    }

    fn set_fan_speed(&self, fans: u16, speed: u16) -> Result<(), Error> {
        let v = format!("{:#04x}", speed);
        for f in 0..=fans {
            let fan = format!("{:#04x}", f);
            self.execute("ipmitool", vec!["raw", "0x30", "0x30", "0x02", &fan, &v])?;
        }
        Ok(())
    }
}

pub(crate) struct IpmiTool {
    cmd: Box<dyn Executer>,
}

impl IpmiTool {
    pub fn new(cmd: Box<dyn Executer>) -> Self {
        Self { cmd }
    }
}

impl Ipmi for IpmiTool {
    fn get_info_fan_temp(&self) -> Result<String, Error> {
        let res = self.cmd.get_info_fan_temp()?;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?i)^fan|temp").unwrap();
        }

        let filtered: String = res
            .lines()
            .filter(|x| RE.is_match(x))
            .map(|x| x.to_owned() + "\n")
            .collect();
        Ok(filtered)
    }

    fn get_cpu_temperature(&self) -> Result<u16, Error> {
        let res = self.cmd.get_cpu_temperature()?;

        lazy_static! {
            static ref RE: Regex = Regex::new(r"(?i)(\d*)\sdegrees").unwrap();
        }

        for x in res.lines() {
            if x.to_lowercase().starts_with("temp_cpu0") {
                if let Some(v) = RE.captures(x) {
                    let tmp = v.get(1).unwrap().as_str().parse::<u16>()?;
                    return Ok(tmp);
                }
            }
        }

        Err(anyhow!("not found"))
    }

    fn set_fan_speed(&self, fans: u16, speed: u16) -> Result<(), Error> {
        self.cmd.set_fan_speed(fans, speed)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct MockCommand {}

    impl Executer for MockCommand {
        fn get_info_fan_temp(&self) -> Result<String, Error> {
            let output = "
Airflow                  | 26 CFM            | ok
Temp_Outlet              | 29 degrees C      | ok
Temp_CPU0                | 35 degrees C      | ok
Temp_CPU1                | 34 degrees C      | ok
Temp_DIMM_AB             | 28 degrees C      | ok
Temp_DIMM_CD             | 28 degrees C      | ok
Temp_DIMM_EF             | 28 degrees C      | ok
Temp_DIMM_GH             | 28 degrees C      | ok
Temp_PCH                 | 36 degrees C      | ok
Temp_VR_CPU0             | 28 degrees C      | ok
Temp_VR_CPU1             | 28 degrees C      | ok
Temp_VR_DIMM_AB          | 29 degrees C      | ok
Temp_VR_DIMM_CD          | 27 degrees C      | ok
Temp_VR_DIMM_EF          | 29 degrees C      | ok
Temp_VR_DIMM_GH          | 27 degrees C      | ok
Temp_PCI1_Outlet         | 31 degrees C      | ok
Temp_PCI2_Outlet         | 35 degrees C      | ok
Temp_Inlet               | 24 degrees C      | ok
Temp_CPU0_Inlet          | 28 degrees C      | ok
Temp_CPU1_Inlet          | 27 degrees C      | ok
Temp_OCP_MEZZ            | 57 degrees C      | ok
Temp_SYS_Outlet          | 29 degrees C      | ok
Fan_SYS0_1               | 8700 RPM          | ok
Fan_SYS0_2               | 7300 RPM          | ok
Fan_SYS1_1               | 8800 RPM          | ok
Fan_SYS1_2               | 7300 RPM          | ok
Fan_SYS2_1               | 8900 RPM          | ok
Fan_SYS2_2               | 7200 RPM          | ok
Fan_SYS3_1               | 9000 RPM          | ok
Fan_SYS3_2               | 7200 RPM          | ok
Fan_SYS4_1               | 8700 RPM          | ok
Fan_SYS4_2               | 7300 RPM          | ok
Volt_P3V3                | 3.35 Volts        | ok
Volt_P5V                 | 5.01 Volts        | ok
Volt_P12V                | 12.12 Volts       | ok
Volt_P1V05               | 1.04 Volts        | ok
Volt_P1V8_AUX            | 1.81 Volts        | ok
Volt_P3V3_AUX            | 3.31 Volts        | ok
Volt_P5V_AUX             | 5.03 Volts        | ok
Volt_P3V_BAT             | 3.13 Volts        | ok
Volt_VR_CPU0             | 1.79 Volts        | ok
Volt_VR_CPU1             | 1.79 Volts        | ok
Volt_VR_DIMM_AB          | 1.21 Volts        | ok
Volt_VR_DIMM_CD          | 1.21 Volts        | ok
Volt_VR_DIMM_EF          | 1.22 Volts        | ok
Volt_VR_DIMM_GH          | 1.22 Volts        | ok
PSU1_Input               | 84 Watts          | ok
PSU2_Input               | 0 Watts           | cr
            ";
            Ok(output.to_string())
        }

        fn get_cpu_temperature(&self) -> Result<String, Error> {
            let output = "
Temp_Outlet              | 1Fh | ok  |  7.1 | 30 degrees C
DCMI Therm Limit | A2h | ns  |  7.31 | Event-Only
Temp_CPU0                | 70h | ok  | 65.1 | 36 degrees C
Temp_CPU1                | 71h | ok  | 65.2 | 34 degrees C
Temp_DIMM_AB             | 72h | ok  | 66.1 | 28 degrees C
Temp_DIMM_CD             | 73h | ok  | 66.2 | 29 degrees C
Temp_DIMM_EF             | 74h | ok  | 66.3 | 29 degrees C
Temp_DIMM_GH             | 75h | ok  | 66.4 | 29 degrees C
Temp_PCH                 | 76h | ok  | 66.5 | 37 degrees C
Temp_VR_CPU0             | 77h | ok  | 66.6 | 28 degrees C
Temp_VR_CPU1             | 78h | ok  | 66.7 | 28 degrees C
Temp_VR_DIMM_AB          | 79h | ok  | 66.8 | 29 degrees C
Temp_VR_DIMM_CD          | 7Ah | ok  | 66.9 | 27 degrees C
Temp_VR_DIMM_EF          | 7Bh | ok  | 66.10 | 29 degrees C
Temp_VR_DIMM_GH          | 7Ch | ok  | 66.11 | 27 degrees C
Temp_PCI1_Outlet         | 7Dh | ok  | 66.12 | 32 degrees C
Temp_PCI2_Outlet         | 7Eh | ok  | 66.13 | 36 degrees C
Temp_Inlet               | 7Fh | ok  | 64.1 | 25 degrees C
Temp_CPU0_Inlet          | A8h | ok  | 66.14 | 28 degrees C
Temp_CPU1_Inlet          | A9h | ok  | 66.15 | 27 degrees C
Temp_OCP_MEZZ            | AAh | ok  | 66.16 | 58 degrees C
Temp_SYS_Outlet          | B5h | ok  | 66.27 | 30 degrees C
PCH Thermal Trip         | BCh | ok  |  7.6 |
MB Thermal Trip          | BDh | ok  |  7.7 |
            ";
            Ok(output.to_string())
        }

        fn set_fan_speed(&self, _fans: u16, _speed: u16) -> Result<(), Error> {
            Ok(())
        }
    }

    #[test]
    fn test_mock_works() {
        let cmd = MockCommand {};
        assert!(cmd.get_info_fan_temp().is_ok());
    }

    #[test]
    fn test_ipmi_tool() {
        let ipmi = IpmiTool::new(Box::new(MockCommand {}));

        let res = ipmi.get_info_fan_temp();
        assert!(res.is_ok());
        println!("{}", res.unwrap());

        let res = ipmi.get_cpu_temperature();
        assert!(res.is_ok());
        assert_eq!(36, res.unwrap());

        let res = ipmi.set_fan_speed(4, 10);
        assert!(res.is_ok());
    }
}
