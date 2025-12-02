use std::fs;
use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct HttpExtension {
    cached_binary_path: Option<String>,
}

impl HttpExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        // Check if we have a cached path
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map(|m| m.is_file()).unwrap_or(false) {
                return Ok(path.clone());
            }
        }

        // Check for user-configured path in settings
        if let Ok(lsp_settings) = LspSettings::for_worktree("http-lsp", worktree) {
            if let Some(binary) = lsp_settings.binary {
                if let Some(path) = binary.path {
                    self.cached_binary_path = Some(path.clone());
                    return Ok(path);
                }
            }
        }

        // Try to find the binary in the extension directory
        let (platform, arch) = zed::current_platform();
        let binary_name = format!(
            "http-lsp-{}-{}{}",
            match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X8664 => "x86_64",
                zed::Architecture::X86 => "x86",
            },
            match platform {
                zed::Os::Mac => "apple-darwin",
                zed::Os::Linux => "unknown-linux-gnu",
                zed::Os::Windows => "pc-windows-msvc.exe",
            },
            match platform {
                zed::Os::Windows => ".exe",
                _ => "",
            }
        );

        // Check if binary exists in extension directory
        let binary_path = format!("./{}", binary_name);
        if fs::metadata(&binary_path).map(|m| m.is_file()).unwrap_or(false) {
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(binary_path);
        }

        // Try to download from GitHub releases
        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release = zed::latest_github_release(
            "tie304/zed-http",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == binary_name)
            .ok_or_else(|| format!("No binary found for platform: {}", binary_name))?;

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::Downloading,
        );

        zed::download_file(
            &asset.download_url,
            &binary_path,
            zed::DownloadedFileType::Uncompressed,
        )?;

        zed::make_file_executable(&binary_path)?;

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for HttpExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_path = self.language_server_binary_path(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary_path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(HttpExtension);
