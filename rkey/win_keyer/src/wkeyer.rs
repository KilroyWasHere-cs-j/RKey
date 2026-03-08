struct KeyerCommand {
    pub name: &'static str,
    pub command: &'static [u8],
    pub description: &'static str,
}

const KEYER_COMMANDS: &[KeyerCommand] = &[
    KeyerCommand {
        name: "HostOpen",
        command: b"\x00\x02",
        description: "Open a connection to the WinKeyer device",
    },
    KeyerCommand {
        name: "HostClose",
        command: b"\x00\x03",
        description: "Close a connection to the WinKeyer device",
    }
];


pub struct WinKeyer {
    //let mut book_reviews = HashMap::new();
}