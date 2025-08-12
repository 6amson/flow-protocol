use serde::de::Expected;
use std::io;
use std::path::{Path, PathBuf};

// use libp2p::{identity::Keypair};
use crate::services::ssid::generate_ed25519_and_save;
use crate::services::user::User;
use crate::utils::errors::FlowError;
use std::fs::{create_dir_all, read, read_dir, write};

#[derive(Debug, Clone, PartialEq)]
pub enum ContentType {
    // Text documents
    Markdown,
    Docx,
    Pdf,
    // Media files
    Mp3,
    Wav,
    Mp4,
    Mov,
    // Code snippets
    JavaScript,
    Python,
    Java,
    Cpp,
    // AI models
    PyTorch,
    Onnx,
    // Unknown but allowed
    Unknown,
}

impl ContentType {
    pub fn from_extension(ext: &str) -> Result<Self, FlowError> {
        match ext.to_lowercase().as_str() {
            "md" => Ok(ContentType::Markdown),
            "docx" => Ok(ContentType::Docx),
            "pdf" => Ok(ContentType::Pdf),
            "mp3" => Ok(ContentType::Mp3),
            "wav" => Ok(ContentType::Wav),
            "mp4" => Ok(ContentType::Mp4),
            "mov" => Ok(ContentType::Mov),
            "js" => Ok(ContentType::JavaScript),
            "py" => Ok(ContentType::Python),
            "java" => Ok(ContentType::Java),
            "cpp" => Ok(ContentType::Cpp),
            "pt" => Ok(ContentType::PyTorch),
            "onnx" => Ok(ContentType::Onnx),
            unsupported => Err(FlowError::ParsingError(format!(
                "Unsupported file type: {}",
                unsupported
            ))),
        }
    }
}

#[derive(Clone)]
pub struct Workspace {
    pub path: PathBuf,
    pub current_user: User,
}

impl Workspace {
    pub fn new(username: &str, base_dir: PathBuf) -> Self {
        let peer_id =
            generate_ed25519_and_save(&base_dir.join("keypair")).expect("Failed to generate peer id");
        let user = User::new(username, &peer_id);
        let path = base_dir.join(format!("flow_{}_workspace", username));
        create_dir_all(&path).map_err(|e| FlowError::FileError(e.to_string())).expect("Dailed");
        Self {
            path,
            current_user: user,
        }
    }

    pub fn create_workspace(base_dir: &PathBuf, username: &str) -> Result<PathBuf, FlowError> {
        let path = base_dir.join(format!("flow_{}_workspace", username));
        create_dir_all(&path).map_err(|e| FlowError::FileError(e.to_string()))?;
        Ok(path)
    }

    pub fn parse_content(&self, file_path: &Path) -> Result<Vec<u8>, FlowError> {
        let payload = read(file_path).map_err(|e| FlowError::FileError(e.to_string()))?;
        if payload.is_empty() {
            return Err(FlowError::FileError("File is empty".to_string()));
        }

        self.validate_supported_format(file_path)?;

        Ok(payload)
    }

    fn validate_supported_format(&self, path: &Path) -> Result<ContentType, FlowError> {
        match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ContentType::from_extension(ext),
            None => Ok(ContentType::Unknown),
        }
    }

    pub fn save_content(&self, data: &[u8], content_id: &str) -> Result<PathBuf, FlowError> {
        let content_dir = self.path.join("content");
        create_dir_all(&content_dir)?;
        let content_path = content_dir.join(content_id);
        write(&content_path, data)?;
        Ok(content_path)
    }

    pub fn get_content(&self, content_id: &str) -> Result<Vec<u8>, FlowError> {
        let content_dir = self.path.join("content");
        let content_path = content_dir.join(content_id);
        let data = read(content_path)?;
        Ok(data)
    }

    pub fn get_content_path(&self, content_id: &str) -> PathBuf {
        self.path.join("content").join(content_id)
    }

    pub fn list_content(&self) -> Result<Vec<String>, FlowError> {
        let content_dir = self.path.join("content");
        let mut contents = Vec::new();

        if content_dir.exists() {
            for entry in read_dir(content_dir)? {
                match entry {
                    Ok(entry) => {
                        let file_name = entry.file_name();
                        let file_name_str = file_name.into_string().unwrap();
                        contents.push(file_name_str);
                    }
                    Err(e) => {
                        eprintln!("Error reading directory entry: {:?}", e);
                    }
                }
            }
        }
        Ok(contents)
    }
}
