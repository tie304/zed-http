use zed_extension_api as zed;

struct HttpExtension {}

impl HttpExtension {}

impl zed::Extension for HttpExtension {
    fn new() -> Self {
        Self {}
    }
    fn language_server_command(
        &mut self,
        _: &zed_extension_api::LanguageServerId,
        _: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        Err(("Not implemented").into())
    }
}

zed::register_extension!(HttpExtension);
