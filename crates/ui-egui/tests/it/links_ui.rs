//! Headless UI test of the project links and the app's name: Help menu commands, the About
//! dialog and the Home (Import) screen.
//!
//! Set `FILMCRAFT_UI_SNAPSHOT_DIR=<dir>` to also render the window offscreen with wgpu and write
//! `links-*.png` there; without it no GPU is needed.

use crate::support::{Drive, Launch, builder, demo_session, driver, save_snapshot, with_snapshots};
use filmcraft_ui_egui::links;
use serde_json::json;

driver!(Driver);

impl Driver {
    fn demo() -> Self {
        Self::open(demo_session(), with_snapshots(builder()))
    }
    fn snapshot(&mut self, name: &str) {
        save_snapshot(self, 2, &format!("links-{name}.png"));
    }
}

#[test]
fn project_links_and_app_name_everywhere() {
    let mut d = Driver::demo();
    // Help menu: every link is a command that reports the URL it opened.
    for (id, _, url) in links::ALL {
        let r = d.ok("engine.execute", json!({"command": id, "params": {}}));
        assert_eq!(r["url"], json!(url), "{id}");
    }
    let menu = d.ok("ui.menu.list", json!({}));
    let text = menu.to_string();
    for (id, label, _) in links::ALL {
        assert!(text.contains(id) && text.contains(label), "Help menu lists {id}");
    }
    // The upstream build's community link commands are gone from the Help menu.
    for gone in ["help.discord", "help.website", "help.appPage"] {
        assert!(!text.contains(gone), "Help menu still lists {gone}");
    }
    // Header: no community button (the upstream build had a Discord button there).
    assert!(d.ids("header.discord").is_empty());
    d.snapshot("header");
    // About dialog: the app's name as plain text where the upstream build drew the brand
    // wordmark, the "Based on" line, then GitHub and issues.
    d.ok("engine.execute", json!({"command": "app.about", "params": {}}));
    d.frames(3);
    let about = d.ids("about.");
    for id in ["about.appName", "about.basedOn", "about.github", "about.reportIssue"] {
        assert!(about.iter().any(|a| a == id), "{id} in {about:?}");
    }
    assert_eq!(d.find_element("about.appName").map(|e| e["label"].clone()), Some(json!("FilmCraft")));
    assert_eq!(d.find_element("about.basedOn").map(|e| e["label"].clone()), Some(json!(filmcraft_ui_egui::brand::BASED_ON)));
    for id in ["about.discord", "about.website", "about.appPage"] {
        assert!(!about.iter().any(|a| a == id), "{id} in {about:?}");
    }
    d.snapshot("about");
    d.ok("ui.key", json!({"key": "Escape"}));
    d.frames(3);
    assert!(d.ids("about.").is_empty(), "Escape closes About");
    // Home screen (Import mode): the app's name as plain text, then the project links.
    d.ok("engine.execute", json!({"command": "mode.import", "params": {}}));
    d.frames(3);
    assert_eq!(d.find_element("import.appName").map(|e| e["label"].clone()), Some(json!("FilmCraft")));
    let home = d.ids("import.link.");
    assert_eq!(home, vec!["import.link.github".to_string(), "import.link.reportIssue".to_string()]);
    d.snapshot("home");
}
