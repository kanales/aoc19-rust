use intcode::Process;

enum MachineStatus {
    Paused,
    Outputing(i32),
    Listening,
    Stop,
}

struct Machine {
    process: Process,
    status: MachineStatus,
}

impl Machine {
    pub fn new(code: Intcode) -> Machine {
        Machine {
            process: Process::new(code),
            status: Paused,
        }
    }
}
