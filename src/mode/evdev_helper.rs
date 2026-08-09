use crate::MainArgs;

pub(crate) fn main(args: &MainArgs) {
    println!("Path\tName\tUnique name\tPhysical path\tBus type");
    for (path, device) in evdev::enumerate() {
        if args.debug {
            dbg!(&device);
        }

        let placeholder = "<no-info>";
        println!(
            "{}\t{}\t{}\t{}\t{}",
            path.display(),
            device.name().unwrap_or(placeholder),
            device.unique_name().unwrap_or(placeholder),
            device.physical_path().unwrap_or(placeholder),
            device.input_id().bus_type(),
        );
    }
}
