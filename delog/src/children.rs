use nix::unistd::Pid;
use sysinfo::System;

pub fn get_child_pids(system: &System, parent_pid: Pid) -> Vec<Pid> {
    let mut child_pids = Vec::new();
    for process in system.processes() {
        if let Some(parent) = process.1.parent() {
            let our_parent : nix::unistd::Pid = parent_pid;
            let this_procs_parent: sysinfo::Pid = parent;

            let our_parent = our_parent.as_raw();
            let this_procs_parent = this_procs_parent.as_u32() as i32;

            if our_parent == this_procs_parent {
                child_pids.push(Pid::from_raw(process.0.as_u32() as i32));
            }
        }
    }
    child_pids
}
