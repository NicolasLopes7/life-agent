use super::models::App;
use appkit_nsworkspace_bindings::{self};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use regex::Regex;
use sysinfo::Pid;
use tokio::process::Command;

pub async fn get_active_proccesses() -> Vec<App> {
    let mut active_apps: Vec<App> = vec![];

    unsafe {
        let shared_workspace: *mut Object = msg_send![class!(NSWorkspace), sharedWorkspace];
        let running_apps: *mut Object = msg_send![shared_workspace, runningApplications];
        let count: usize = msg_send![running_apps, count];

        for i in 0..count {
            let app: *mut Object = msg_send![running_apps, objectAtIndex: i];

            if app.is_null() {
                continue;
            }

            let pid: i32 = msg_send![app, processIdentifier];

            if pid == -1 {
                continue;
            }

            let activation_policy: isize = msg_send![app, activationPolicy];
            let pid = Pid::from_u32(pid as u32);

            if activation_policy != 0 {
                continue;
            }

            active_apps.push(App { pid, name: None });
        }
    }

    for i in 0..active_apps.len() {
        let cmd = Command::new("ps")
            .arg(active_apps[i].pid.to_string())
            .arg("-o command")
            .output()
            .await
            .expect("failed to get process name");

        let string_output = String::from_utf8(cmd.stdout).unwrap();

        let pattern = r#"(?:\/)([A-z\s\-]*).(?:app)"#;
        let regex = Regex::new(pattern).unwrap();

        match regex.captures(string_output.as_str()) {
            Some(cap) => active_apps[i].name = Some(cap.get(1).unwrap().as_str().to_string()),
            None => {}
        }
    }

    return active_apps;
}
