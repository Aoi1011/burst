use std::{collections::HashMap, io};
use rusoto::{DefaultCredentialProvider, Region};
use rusoto_dynamodb::{DynamodbClient, ListTablesInput};

struct SshConnection;

struct Machine {
    ssh: SshConnection,
    instance_type: String,
    ip: String,
    dns: String,
}

pub struct MachineSetup<F> {
    instance_type: String,
    ami: String,
    setup: F,
}

impl MachineSetup<F> {
    pub fn new<F>(instance_type: String, ami: String, setup: F) -> Self
    where
        F: Fn(&mut SshConnection) -> io::Result<()>,
    {
        MachineSetup {
            instance_type,
            ami,
            setup,
        }
    }
}

struct BurstBuilder {
    descriptors: HashMap<String, (MachineSetup, u32)>,
}

impl Default for BurstBuilder {
    fn default() -> Self {
        BurstBuilder {
            descriptors: Default::default(),
        }
    }
}

impl BurstBuilder {
    pub fn add_set(&mut self, name: String, number: u32, setup: MachineSetup) {
        // TODO: what if name is already in use?
        self.descriptors.insert(name, (setup, number));
    }

    pub fn run<F>() where F: FnOnce(HashMap<String, &mut [Machine]>) -> io::Result<()> {
        // 1. issue spot requests
        // 2. wait for instace to come up
        //  - once an instance is ready, run setup closure
        // 3. wait until all instances are up and setups have been run
        // 4. stop spot request
        // 5. invoke F with Machine descriptors
        // 6. terminate all instances
    }
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
