use std::process::Command;

pub(super) enum PickerKind {
    Folder,
    Executable,
    Csv,
    Stampworld,
}

fn picker_command(kind: PickerKind, title: &str, start: &str) -> Command {
    let mut command = Command::new("powershell.exe");
    command
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-STA",
            "-Command",
            include_str!("windows_picker.ps1"),
        ])
        .env("STAMP_PICKER_TITLE", title)
        .env("STAMP_PICKER_START", start)
        .env(
            "STAMP_PICKER_KIND",
            match kind {
                PickerKind::Folder => "folder",
                PickerKind::Executable => "executable",
                PickerKind::Csv => "csv",
                PickerKind::Stampworld => "stampworld",
            },
        );
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW; the dialog remains visible.
    }
    command
}

fn decode_selection(
    status: Option<i32>,
    stdout: &[u8],
    stderr: &[u8],
) -> Result<Option<String>, String> {
    match status {
        Some(0) => {
            let selected: String = serde_json::from_slice(stdout)
                .map_err(|e| format!("Windows picker returned an invalid path: {}", e))?;
            if selected.is_empty() {
                return Err("Windows picker returned an empty path".to_string());
            }
            Ok(Some(selected))
        }
        Some(10) => Ok(None),
        _ => Err(format!(
            "Windows picker failed (exit {:?}): {}",
            status,
            String::from_utf8_lossy(stderr).trim()
        )),
    }
}

#[cfg(windows)]
pub(super) fn pick(kind: PickerKind, title: &str, start: &str) -> Result<Option<String>, String> {
    let output = picker_command(kind, title, start)
        .output()
        .map_err(|e| format!("Failed to launch Windows picker (powershell.exe): {}", e))?;
    decode_selection(output.status.code(), &output.stdout, &output.stderr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_picker_uses_sta_and_passes_inputs_as_literal_environment_values() {
        let title = "Select CSV for O'Brien; $country";
        let start = r"C:\Stamp Albums\O'Brien & Sons";
        for (kind, expected) in [
            (PickerKind::Folder, "folder"),
            (PickerKind::Executable, "executable"),
            (PickerKind::Csv, "csv"),
            (PickerKind::Stampworld, "stampworld"),
        ] {
            let command = picker_command(kind, title, start);
            assert_eq!(command.get_program(), "powershell.exe");
            let args: Vec<_> = command.get_args().collect();
            assert!(args.contains(&std::ffi::OsStr::new("-STA")));
            assert_eq!(
                args.last().copied(),
                Some(std::ffi::OsStr::new(include_str!("windows_picker.ps1")))
            );
            let env: std::collections::HashMap<_, _> = command.get_envs().collect();
            assert_eq!(
                env[std::ffi::OsStr::new("STAMP_PICKER_TITLE")],
                Some(std::ffi::OsStr::new(title))
            );
            assert_eq!(
                env[std::ffi::OsStr::new("STAMP_PICKER_START")],
                Some(std::ffi::OsStr::new(start))
            );
            assert_eq!(
                env[std::ffi::OsStr::new("STAMP_PICKER_KIND")],
                Some(std::ffi::OsStr::new(expected))
            );
        }
    }

    #[test]
    fn windows_picker_preserves_unicode_spaces_and_unc_paths() {
        for path in [
            "C:\\Stamp Albums\\Espa\u{00f1}a\\",
            "\\\\server\\share\\Stamp Albums\\stamps.csv",
            "C:\\O'Brien & Sons\\AlbumEasy.exe",
        ] {
            let json = serde_json::to_vec(path).expect("serialize path");
            assert_eq!(
                decode_selection(Some(0), &json, b"").expect("selected path"),
                Some(path.to_string())
            );
        }
    }

    #[test]
    fn windows_picker_distinguishes_cancellation_from_failure() {
        assert_eq!(decode_selection(Some(10), b"", b""), Ok(None));
        assert!(decode_selection(Some(1), b"", b"PowerShell startup failed").is_err());
        let error = decode_selection(Some(2), b"", b"dialog unavailable")
            .expect_err("dialog failure must be reported");
        assert!(error.contains("dialog unavailable"));
        assert!(decode_selection(None, b"", b"terminated").is_err());
    }

    #[test]
    fn windows_picker_rejects_invalid_success_output() {
        for output in [
            b"".as_slice(),
            b"null",
            b"\"\"",
            b"not JSON",
            b"\"bad\xff\"",
        ] {
            assert!(decode_selection(Some(0), output, b"").is_err());
        }
    }
}
