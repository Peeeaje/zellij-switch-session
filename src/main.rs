use std::collections::BTreeMap;
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
        let dir = self
            .configuration
            .get("dir")
            .expect("Expected 'dir' in configuration");
        switch_session_with_layout(
            self.configuration.get("session_name").map(|s| s.as_str()),
            LayoutInfo::File("default".to_string()),
            Some(std::path::PathBuf::from(dir)),
        );
        close_self();
    }
}
