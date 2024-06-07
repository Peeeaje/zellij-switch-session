use std::collections::BTreeMap;
use std::path::PathBuf;
use zellij_tile::prelude::*;

#[derive(Default)]
struct Switcher {
    configuration: BTreeMap<String, String>,
    permissions_granted: bool,
}

register_plugin!(Switcher);
impl ZellijPlugin for Switcher {
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        self.configuration = configuration;

        request_permission(&[PermissionType::ChangeApplicationState]);
        subscribe(&[EventType::PermissionRequestResult]);

        if self.permissions_granted {
            hide_self();
        }
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::PermissionRequestResult(permission) => {
                self.permissions_granted = match permission {
                    PermissionStatus::Granted => true,
                    PermissionStatus::Denied => false,
                };
                if self.permissions_granted {
                    hide_self();
                }
            }
            _ => {}
        }
        false
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if let Some(payload) = pipe_message.payload {
            let mut collection = payload.splitn(2, "::");
            if let (Some(session_name), Some(cwd)) = (collection.next(), collection.next()) {
                let layout: LayoutInfo = LayoutInfo::File("default".to_string());
                switch_session_with_layout(Some(session_name), layout, Some(PathBuf::from(cwd)));
            }
        }
        false
    }

    fn render(&mut self, _: usize, _: usize) {
        let cwd = self.configuration.get("dir").map(|d| PathBuf::from(d));
        let session_name = self.configuration.get("session_name").map(|s| s.as_str());
        switch_session_with_layout(session_name, LayoutInfo::File("default".to_string()), cwd);
        close_self();
    }
}
