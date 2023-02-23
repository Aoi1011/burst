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
