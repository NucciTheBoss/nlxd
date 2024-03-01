/// Example of creating an instance.
/// Requires a local LXD running.
use nlxd::client::Client;
use nlxd::instance::NewInstance;

fn main() {
    let client = Client::new(Default::default()).unwrap();
    let spec = NewInstance::default_with_image("juju/ubuntu@20.04/amd64".to_owned());
    let new_instance_response = client.create_instance(&spec).unwrap();
    dbg!(new_instance_response);
}
