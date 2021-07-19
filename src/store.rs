use serde::{Serialize, Deserialize};
use crate::client::LinkboxClient;
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct StoreData {
    base_url: String,
    jwt: String,
}

pub fn load_from_store() -> std::io::Result<Option<LinkboxClient>> {
    let mut store_file = home::home_dir().expect("User does not appear to have home directory");
    store_file.push(".lbcli.json");

    if store_file.exists() {
        let mut file = File::open(store_file)?;
        let store: StoreData = serde_json::from_reader(&mut file)?;
        let client = LinkboxClient::new(store.base_url)
            .with_jwt(store.jwt);
        Ok(Some(client))
    } else {
        Ok(None)
    }
}

pub fn save_to_store(client: &LinkboxClient) -> std::io::Result<bool> {
    if let Some(jwt) = client.get_jwt() {
        let mut store_file = home::home_dir().expect("User does not appear to have home directory");
        store_file.push(".lbcli.json");

        let file = File::create(store_file)?;
        serde_json::to_writer(file, &StoreData {
            jwt,
            base_url: client.get_base_url(),
        })?;

        Ok(true)
    } else {
        Ok(false)
    }
}

pub fn remove_store() -> std::io::Result<()> {
    let mut store_file = home::home_dir().expect("User does not appear to have a home directory!");
    store_file.push(".lbcli.json");

    std::fs::remove_file(store_file)?;

    Ok(())
}
