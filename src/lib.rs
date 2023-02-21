use std::{collections::HashMap, io};

struct SshConnection {}

struct MachineSetup {
    instance_type: String,
    ami: String,
}

impl MachineSetup {
    pub fn new<F>(instance_type: String, ami: String, setup: F) -> Self
    where
        F: Fn(&mut SshConnection) -> io::Result<()>,
    {
        MachineSetup {}
    }
}

struct BurstBuilder {
    descriptors: Vec<MachineSetup>,
}

impl Default for BurstBuilder {
    fn default() -> Self {
        BurstBuilder {
            descriptors: Vec::new(),
        }
    }
}

impl BurstBuilder {
    pub fn add_set(&mut self, name: String, number: u32, description: MachineSetup) {}

    pub fn run() {}
}

fn main() {
    let mut b = BurstBuilder::default();
    b.add_set(
        "server",
        1,
        MachineSetup::new("t2.micro", "ami-e18aa89b", |ssh| {
            ssh.exec("sudo yum install htop");
        }),
    );
    b.add_set(
        "client",
        2,
        MachineSetup::new("t2.micro", "ami-e18aa89b", |ssh| {
            ssh.exec("sudo yum install htop");
        }),
    );
    b.run(|vms: HashMap<String, MachineSetup>| {
        let server_ip = vms["server"][0].ip;
        let cmd = format!("ping {}", server_ip);
        vms["client"].for_each_parallel(|client| {
            client.exec(cmd);
        });
    })
}
