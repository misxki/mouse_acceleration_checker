extern crate winrt_notification;
use std::path::Path;

use winrt_notification::{Duration, Toast, IconCrop};

fn is_pointer_precision_on() -> bool {
    let regkey: registry::RegKey = registry::Hive::CurrentUser.open(r"Control Panel\Mouse", registry::Security::Read).unwrap();

    let mouse_speed_value: u16 = regkey.value("MouseSpeed").unwrap().to_string().parse::<u16>().unwrap();
    let mouse_threshold1_value: u16 = regkey.value("MouseThreshold1").unwrap().to_string().parse::<u16>().unwrap();
    let mouse_threshold2_value: u16 = regkey.value("MouseThreshold2").unwrap().to_string().parse::<u16>().unwrap();

    !(mouse_speed_value == 0 && mouse_threshold1_value == 0 && mouse_threshold2_value == 0)
}

fn main()
{

    let is_pointer_precision_on: bool = is_pointer_precision_on();
    let pointer_precision_text: &str;

    if is_pointer_precision_on {
        pointer_precision_text = "Mouse Acceleration is on";
    } else {
        pointer_precision_text = "Mouse Acceleration is off";
    }

    Toast::new(Toast::POWERSHELL_APP_ID)
        //.hero(&Path::new("kek"), "MA")
        //.icon(&Path::new("kek"), IconCrop::Square, "MAicon")
        .title("Mouse Acceleration")
        .text1("(╯°□°）╯︵ ┻━┻")
        .text2(&pointer_precision_text)
        .sound(None)
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");

}