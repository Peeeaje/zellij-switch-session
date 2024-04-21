use std::collections::BTreeMap;
use std::path::PathBuf;
use zellij_tile::prelude::*;

#[derive(Default)]
struct Switcher {
    configuration: BTreeMap<String, String>,
}

register_plugin!(Switcher);
impl ZellijPlugin for Switcher {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        request_permission(&[PermissionType::ChangeApplicationState]);
        self.configuration = configuration;
    }

    fn update(&mut self, _: Event) -> bool {
        false
    }

    fn render(&mut self, _: usize, _: usize) {
        let cwd = self.configuration.get("dir").map(|d| PathBuf::from(d));
        let session_name = self.configuration.get("session_name").map(|s| s.as_str());
        switch_session_with_layout(session_name, LayoutInfo::File("default".to_string()), cwd);
        close_self();
    }
}
