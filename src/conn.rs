use crate::subsonic::{self, Id, SubsonicData, SubsonicResponse};

use color_eyre::Result;
use md5::Digest;
use secrecy::{ExposeSecret, Secret};
use ureq::Request;

const SUBSONIC_API_VERSION: &str = "1.16.1";
const SUBSONIC_CLIENT_NAME: &str = "navicon";

pub struct Connection {
    url: String,
    user: String,
    password: Secret<String>,
}

impl Connection {
    pub fn new(url: String, user: String, password: String) -> Connection {
        Connection {
            url,
            user,
            password: password.into(),
        }
    }

    pub fn ping(&self) -> Result<bool> {
        let req = ApiRequest::build(self, "ping");
        let resp: SubsonicResponse = req.call()?;
        Ok(resp.status == "ok")
    }

    pub fn get_license(&self) -> Result<SubsonicResponse> {
        let req = ApiRequest::build(self, "getLicense");
        req.call()
    }

    pub fn get_music_folders(&self) -> Result<SubsonicResponse> {
        let req = ApiRequest::build(self, "getMusicFolders");
        req.call()
    }

    pub fn get_artists(&self) -> Result<SubsonicResponse> {
        let req = ApiRequest::build(self, "getArtists");
        req.call()
    }

    pub fn get_artist(&self, id: &Id) -> Result<SubsonicResponse> {
        let req = ApiRequest::build(self, "getArtist").param("id", id);
        req.call()
    }

    pub fn get_album(&self, id: &Id) -> Result<SubsonicResponse> {
        let req = ApiRequest::build(self, "getAlbum").param("id", id);
        req.call()
    }
}

struct ApiRequest {
    request: Request,
}

impl ApiRequest {
    fn build(connection: &Connection, endpoint: &str) -> ApiRequest {
        let url = format!("{}/rest/{}", connection.url, endpoint);

        let salt = format!("{:x}", rand::random::<u64>());
        let token = format!(
            "{:032x}",
            md5::Md5::digest(format!("{}{}", connection.password.expose_secret(), salt))
        );

        // let salt_test = "c19b2d";
        // let token_test = format!("{:032x}", md5::Md5::digest(format!("sesame{}", salt_test)));
        // assert_eq!(token_test, "26719a1196d2a940705a59634eb18eab");

        let request = ureq::get(&url)
            .query("v", SUBSONIC_API_VERSION)
            .query("c", SUBSONIC_CLIENT_NAME)
            .query("u", &connection.user)
            .query("s", &salt)
            .query("t", &token);

        ApiRequest { request }
    }

    fn param(self, param: &str, value: &str) -> Self {
        ApiRequest {
            request: self.request.query(param, value),
        }
    }

    fn call(self) -> Result<SubsonicResponse> {
        let resp = self.request.call()?;
        let body = resp.into_string()?;
        let parsed_resp: SubsonicResponse = serde_xml_rs::from_str(&body)?;
        if let Some(SubsonicData::Error(err)) = &parsed_resp.data {
            Err(ConnectionError::from(err).into())
        } else {
            Ok(parsed_resp)
        }
    }
}

#[derive(Debug)]
pub struct ConnectionError {
    pub code: ConnectionErrorType,
    pub msg: Option<String>,
}

impl std::error::Error for ConnectionError {}

impl std::fmt::Display for ConnectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = &self.msg {
            write!(f, "{:?} : {}", self.code, msg)
        } else {
            write!(f, "{:?}", self.code)
        }
    }
}

#[derive(Debug)]
pub enum ConnectionErrorType {
    Generic,
    MissingParameter,
    IncompatibleClient,
    IncompatibleServer,
    WrongCredentials,
    NotAuthorized,
    NotFound,
    Other(usize),
}

impl ConnectionError {
    fn from(value: &subsonic::Error) -> Self {
        ConnectionError {
            code: match value.code {
                0 => ConnectionErrorType::Generic,
                10 => ConnectionErrorType::MissingParameter,
                20 => ConnectionErrorType::IncompatibleClient,
                30 => ConnectionErrorType::IncompatibleServer,
                40 => ConnectionErrorType::WrongCredentials,
                50 => ConnectionErrorType::NotAuthorized,
                70 => ConnectionErrorType::NotFound,
                _ => ConnectionErrorType::Other(value.code),
            },
            msg: value.message.clone(),
        }
    }
}
