use reqwest::{Client, StatusCode};
use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::model::{Link, ListLinksData, GetLinkData, CreateLink, PartialLink, DiscoveryData};

#[derive(Debug)]
pub struct LinkboxClient {
    client: Client,
    base_url: String,
    jwt: Option<String>,
}

impl LinkboxClient {
    pub async fn is_valid_instance(base_url: &str) -> Result<bool, Box<dyn Error>> {
        let client = Client::builder()
            .user_agent(USER_AGENT)
            .build().unwrap();

        let req = client.get(format!("{}/api/v1/_lb-discover", base_url))
            .build()?;
        let res = client.execute(req).await?;
        let code = res.status();
        if !code.is_success() {
            return Ok(false);
        }

        let res: DiscoveryData = res.json().await?;

        Ok(res.link == "box")
    }

    pub fn new(mut base_url: String) -> Self {
        if base_url.ends_with('/') {
            base_url.remove(base_url.len() - 1);
        }

        Self {
            client: Client::builder()
                .user_agent(USER_AGENT)
                .build().unwrap(),
            jwt: None,
            base_url,
        }
    }

    pub fn with_jwt(mut self, jwt: String) -> LinkboxClient {
        self.jwt = Some(jwt);
        self
    }

    pub async fn login(&mut self, username: String, password: String) -> Result<(), Box<dyn Error>> {
        let req = self.client.post(format!("{}/api/v1/signin", self.base_url))
            .json(&SignInRequest {
                username,
                password,
            })
            .build()?;
        let res = self.client.execute(req).await?;
        let code = res.status();
        if !code.is_success() {
            return Err(AuthError::InvalidCredentials("signin".to_string()).into());
        }

        let res: JwtResponse = res.json().await?;
        self.jwt = Some(res.jwt);

        Ok(())
    }

    pub async fn list_links(&mut self) -> Result<Vec<Link>, Box<dyn Error>> {
        if self.jwt.is_none() {
            Err(AuthError::NotAuthorized("list links".to_string()).into())
        } else {
            let req = self.client.get(format!("{}/api/v1/links", self.base_url))
                .bearer_auth(self.jwt.as_ref().unwrap())
                .build()?;
            let res = self.client.execute(req).await?;
            let code = res.status();
            if !code.is_success() {
                return Err(AuthError::NotAuthorized("list links".to_string()).into());
            }
            let res: ListLinksData = res.json().await?;

            Ok(res.data)
        }
    }

    pub async fn fetch_link(&mut self, id: i32) -> Result<Option<Link>, Box<dyn Error>> {
        if self.jwt.is_none() {
            Err(AuthError::NotAuthorized("get link".to_string()).into())
        } else {
            let req = self.client.get(format!("{}/api/v1/links/{}", self.base_url, id))
                .bearer_auth(self.jwt.as_ref().unwrap())
                .build()?;
            let res = self.client.execute(req).await?;
            let code = res.status();
            if code == StatusCode::NOT_FOUND {
                Ok(None)
            } else if !code.is_success() {
                Err(AuthError::NotAuthorized("get link".to_string()).into())
            } else {
                let res: GetLinkData = res.json().await?;
                Ok(Some(res.data))
            }
        }
    }

    pub async fn create_link(&mut self, url: String, note: String) -> Result<i32, Box<dyn Error>> {
        if self.jwt.is_none() {
            Err(AuthError::NotAuthorized("create link".to_string()).into())
        } else {
            let req = self.client.post(format!("{}/api/v1/links", self.base_url))
                .bearer_auth(self.jwt.as_ref().unwrap())
                .json(&CreateLink {
                    link: PartialLink {
                        url,
                        note,
                    },
                })
                .build()?;
            let res = self.client.execute(req).await?;
            let code = res.status();
            if !code.is_success() {
                Err(AuthError::ServerError(code.as_u16()).into())
            } else {
                let res: GetLinkData = res.json().await?;
                Ok(res.data.id)
            }
        }
    }

    pub async fn delete_link(&mut self, id: i32) -> Result<(), Box<dyn Error>> {
        if self.jwt.is_none() {
            Err(AuthError::NotAuthorized("delete link".to_string()).into())
        } else {
            let req = self.client.delete(format!("{}/api/v1/links/{}", self.base_url, id))
                .bearer_auth(self.jwt.as_ref().unwrap())
                .build()?;
            let res = self.client.execute(req).await?;
            let code = res.status();
            if !code.is_success() {
                Err(AuthError::ServerError(code.as_u16()).into())
            } else {
                Ok(())
            }
        }
    }

    pub fn get_base_url(&self) -> String {
        self.base_url.clone()
    }

    pub fn get_jwt(&self) -> Option<String> {
        self.jwt.clone()
    }
}

#[derive(Serialize, Deserialize)]
struct SignInRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
struct JwtResponse {
    jwt: String,
}

pub const USER_AGENT: &str = concat!(
env!("CARGO_PKG_NAME"),
"/",
env!("CARGO_PKG_VERSION"),
" (https://github.com/ashisbored/linkbox-cli)",
);

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("invalid credentials")]
    InvalidCredentials(String),
    #[error("not authorized")]
    NotAuthorized(String),
    #[error("server error: {0}")]
    ServerError(u16),
}
