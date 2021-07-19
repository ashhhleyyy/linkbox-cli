use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct DiscoveryData {
    pub link: String,
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    pub id: i32,
    pub url: String,
    pub note: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartialLink {
    pub url: String,
    pub note: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListLinksData {
    pub data: Vec<Link>,
}

#[derive(Serialize, Deserialize)]
pub struct GetLinkData {
    pub data: Link,
}

#[derive(Serialize, Deserialize)]
pub struct CreateLink {
    pub link: PartialLink,
}
