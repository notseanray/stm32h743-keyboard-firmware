pub const NORTH_DOWN: u8 = 1; // (1 or 0) If the magnets in your stems are north side down or south side down
// How far (in millivolts) a key needs to move down before it's considered pressed
pub const ACTUATION_THRESHOLD: u16 = 20; // 10-80 or so.
// How far it needs to move back up before it's considered released
pub const RELEASE_THRESHOLD: u16 = 5; // 5-50 or so; Prevents bouncing by implementing some hysteresis
// Number of analog multiplexers on this keyboard
pub const NUM_MULTIPLEXERS: usize = 5;
// Maximum number of channels on each multiplexer/remote control
pub const MAX_CHANNELS: usize = 21; // Multiplexers are always 16 but my remote has 21 buttons 🤷
// USB identifiers (these are for a generic keyboard)
pub const USB_VID: u16 = 0x16c0;
pub const USB_PID: u16 = 0x27db;
// So we don't bother with disconnected pins, all mV values below this are ignored
pub const IGNORE_BELOW: u16 = 60; // Probably leave this alone; just saves a smidge of CPU time
// Don't touch keyboard stuff below this point unless you know what you're doing
pub const RECALIBRATION_RATE: u32 = 1; // How often to recalibrate all switches (seconds)
