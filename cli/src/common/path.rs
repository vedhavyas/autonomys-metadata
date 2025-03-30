use anyhow::{bail, Context};
use std::convert::TryFrom;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) struct QrPath {
    pub(crate) dir: PathBuf,
    pub(crate) file_name: QrFileName,
}

impl QrPath {
    pub(crate) fn to_path_buf(&self) -> PathBuf {
        self.dir.join(self.file_name.to_string())
    }
}

impl TryFrom<&PathBuf> for QrPath {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        Ok(Self {
            dir: path.parent().unwrap().to_path_buf(),
            file_name: QrFileName::try_from(path)?,
        })
    }
}

impl fmt::Display for QrPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.dir.join(self.file_name.to_string()).to_str().unwrap()
        )
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) enum ContentType {
    Metadata(u32),
    Specs,
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ContentType::Metadata(version) => write!(f, "metadata_{version}"),
            ContentType::Specs => write!(f, "specs"),
        }
    }
}

impl TryFrom<&str> for ContentType {
    type Error = anyhow::Error;

    fn try_from(content_type: &str) -> Result<Self, Self::Error> {
        if content_type.ends_with("specs") {
            return Ok(Self::Specs);
        }
        let mut split = content_type.rsplit('_');
        match (split.next(), split.next()) {
            (Some(version), Some("metadata")) => Ok(Self::Metadata(version.parse()?)),
            _ => bail!("unable to parse content type {}", content_type),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub(crate) struct QrFileName {
    pub(crate) chain: String,
    pub(crate) content_type: ContentType,
    extension: Option<String>,
}

impl QrFileName {
    pub(crate) fn new(chain: &str, content_type: ContentType) -> Self {
        let extension = match content_type {
            ContentType::Metadata(_) => "apng",
            ContentType::Specs => "png",
        };

        QrFileName {
            chain: chain.to_owned(),
            content_type,
            extension: Some(extension.to_string()),
        }
    }
}

impl TryFrom<&PathBuf> for QrFileName {
    type Error = anyhow::Error;

    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        let extension = path.extension().map(|s| s.to_str().unwrap().to_owned());
        let filename = path.file_stem().unwrap().to_str().unwrap();

        let content_type = ContentType::try_from(filename).context("error parsing context type")?;
        let chain = filename
            .strip_suffix(&format!("_{content_type}"))
            .context("error parsing chain name")?;

        Ok(Self {
            chain: String::from(chain),
            content_type,
            extension,
        })
    }
}

impl fmt::Display for QrFileName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let file_name = format!("{}_{}", self.chain, self.content_type);
        match &self.extension {
            Some(ext) => write!(f, "{file_name}.{ext}"),
            None => write!(f, "{file_name}"),
        }
    }
}
