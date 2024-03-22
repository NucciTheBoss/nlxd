/// Example of listing images.
/// Requires a local LXD running.
use nlxd::client::Client;

fn main() {
    let client = Client::new(Default::default()).unwrap();
    let image_fingerprints: Vec<String> = client.images().unwrap();
    println!("{:#?}", &image_fingerprints);
    for fingerprint in image_fingerprints {
        println!("{:#?}", client.get_image(&fingerprint).unwrap());
    }
}
