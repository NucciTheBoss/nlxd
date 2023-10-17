use nlxd::client::get_server_info;

fn main() {
    // If you have latest lxd snap running and your user is in the lxd group, this should work
    println!(
        "{}",
        get_server_info("unix:/var/snap/lxd/common/lxd-user/unix.socket")
    );
    // This should only work if you have lxd running and configured to listen on a port.
    // println!("{}", get_server_info("http://localhost:8080"));
}
