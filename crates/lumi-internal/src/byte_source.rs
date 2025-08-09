use std::{fs, io::{self, Read}, path::PathBuf, str::FromStr, sync::Arc};

use log::error;
#[cfg(feature = "reqwest")]
use reqwest::IntoUrl;

#[derive(Debug, Clone)]
pub enum ByteSource {
    Bytes(Arc<[u8]>),
    File(PathBuf),
    #[cfg(feature = "ureq")]
    Ureq(String),
    #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Url)
}

impl ByteSource {
    pub fn bytes(bytes: impl AsRef<[u8]>) -> Self {
        Self::Bytes(bytes.as_ref().into())
    }

    pub fn from_path(path: PathBuf) -> Self {
        Self::File(path)
    }

    pub fn file(path: &str) -> Option<Self> {
        let parsed = PathBuf::from_str(path).ok()?;

        match parsed.try_exists() {
            Ok(true) => Some(Self::File(parsed)),
            Ok(false) => {
                error!("File at {} does not exist", parsed.to_string_lossy());
                None
            },
            Err(err) => {
                error!("Failed to check if file {} exists: {err}", parsed.to_string_lossy());
                None
            }
        }
    }

    pub fn file_unchecked(path: &str) -> Self {
        Self::from_path(PathBuf::from_str(path).unwrap())
    }

    #[cfg(any(feature = "reqwest", feature = "ureq"))]
    pub fn url(url: impl AsRef<str>) -> Option<Self> {
        #[cfg(feature = "ureq")]
        return Some(Self::ureq_url(url.as_ref().to_string()));
        #[cfg(feature = "reqwest")]
        return Self::reqwest_url(url.as_ref()).ok();
    }

    #[cfg(feature = "ureq")]
    pub fn ureq_url(url: String) -> Self {
        Self::Ureq(url)
    }

    #[cfg(feature = "reqwest")]
    pub fn reqwest_url(url: impl IntoUrl) -> Result<Self, reqwest::Error> {
        Ok(Self::Reqwest(url.into_url()?))
    }


    pub async fn get(&self) -> Result<Arc<[u8]>, SourceError> {
        match self {
            ByteSource::Bytes(arc) => Ok(arc.clone()),
            ByteSource::File(path) => {
                let bytes = fs::read(path)
                .map_err(SourceError::Io)?;

                Ok(bytes.into())
            },
            #[cfg(feature = "ureq")]
            ByteSource::Ureq(url) => {
                let mut bytes = Vec::new();

                crate::UREQ_CLIENT
                .get(url)
                .call()
                .map_err(SourceError::Ureq)?
                .into_reader()
                .take(1024 * 1024 * 1024) // Take at most 1GiB worth of bytes
                .read_to_end(&mut bytes)
                .map_err(SourceError::Io)?;
                
                Ok(bytes.into())
            },
            #[cfg(feature = "reqwest")]
            ByteSource::Reqwest(url) => {
                let bytes = crate::REQWEST_CLIENT
                .get(url.clone())
                .send()
                .await
                .map_err(SourceError::Reqwest)?
                .bytes()
                .await
                .map_err(SourceError::Reqwest)?;

                Ok(Arc::from_iter(bytes))
            }
        }
    }
}

#[derive(Debug)]
pub enum SourceError {
    Io(io::Error),
    #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Error),
    #[cfg(feature = "ureq")]
    Ureq(ureq::Error)
}

impl Default for ByteSource {
    fn default() -> Self {
        Self::Bytes(Arc::new([]))
    }
}

impl From<&[u8]> for ByteSource {
    fn from(bytes: &[u8]) -> Self {
        Self::bytes(bytes)
    }
}