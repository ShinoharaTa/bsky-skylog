use aerostream::Client;

fn main() {
    let mut client = Client::default();
    client.set_timeout(5);
    let did = match client.get_handle("shino3.net") {
        Ok(r) => r.clone(),
        _ => String::from("UNKNOWN"),
    };
    let handle = client.get_repo(did);
    println!("{:?}", handle);
}
