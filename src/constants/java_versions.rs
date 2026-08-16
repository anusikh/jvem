//! Catalog of JDK distributions available for install.
//!
//! Each [`Jdk`] entry carries the download URL for every platform the
//! distribution ships, so a link can be resolved for the current
//! platform without a stringly-typed lookup table.

/// A downloadable JDK distribution.
#[derive(Debug, Clone, Copy)]
pub struct Jdk {
    /// Canonical (uppercase) identifier used on the CLI, e.g. `ZULU17`.
    pub name: &'static str,
    pub linux_x64: &'static str,
    pub linux_aarch64: Option<&'static str>,
    pub macos_x64: &'static str,
    pub macos_aarch64: Option<&'static str>,
    pub windows: &'static str,
}

/// All JDK distributions available for install, in display order.
pub static JDKS: &[Jdk] = &[
    // zulu jdk's
    Jdk {
        name: "ZULU8",
        linux_x64: "https://cdn.azul.com/zulu/bin/zulu8.78.0.19-ca-jdk8.0.412-linux_x64.zip",
        linux_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu8.78.0.19-ca-jdk8.0.412-linux_aarch64.tar.gz",
        ),
        macos_x64: "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-macosx_x64.tar.gz",
        macos_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-macosx_aarch64.tar.gz",
        ),
        windows: "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-win_x64.zip",
    },
    Jdk {
        name: "ZULU11",
        linux_x64: "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-linux_x64.tar.gz",
        linux_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-linux_aarch64.tar.gz",
        ),
        macos_x64: "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-macosx_x64.tar.gz",
        macos_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-macosx_aarch64.tar.gz",
        ),
        windows: "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-win_x64.zip",
    },
    Jdk {
        name: "ZULU17",
        linux_x64: "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-linux_x64.tar.gz",
        linux_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-linux_aarch64.tar.gz",
        ),
        macos_x64: "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-macosx_x64.tar.gz",
        macos_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-macosx_aarch64.tar.gz",
        ),
        windows: "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-win_x64.zip",
    },
    Jdk {
        name: "ZULU21",
        linux_x64: "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-linux_x64.tar.gz",
        linux_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-linux_aarch64.tar.gz",
        ),
        macos_x64: "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-macosx_x64.tar.gz",
        macos_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-macosx_aarch64.tar.gz",
        ),
        windows: "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-win_x64.zip",
    },
    Jdk {
        name: "ZULU22",
        linux_x64: "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-linux_x64.tar.gz",
        linux_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-linux_aarch64.tar.gz",
        ),
        macos_x64: "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-macosx_x64.tar.gz",
        macos_aarch64: Some(
            "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-macosx_aarch64.tar.gz",
        ),
        windows: "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-win_x64.zip",
    },
    // openjdk jdk's
    Jdk {
        name: "OPENJDK11",
        linux_x64: "https://download.java.net/java/GA/jdk11/13/GPL/openjdk-11.0.1_linux-x64_bin.tar.gz",
        linux_aarch64: None,
        macos_x64: "https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_osx-x64_bin.tar.gz",
        macos_aarch64: None,
        windows: "https://download.java.net/java/GA/jdk11/13/GPL/openjdk-11.0.1_windows-x64_bin.zip",
    },
    Jdk {
        name: "OPENJDK17",
        linux_x64: "https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz",
        linux_aarch64: Some(
            "https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-aarch64_bin.tar.gz",
        ),
        macos_x64: "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_macos-x64_bin.tar.gz",
        macos_aarch64: Some(
            "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_macos-aarch64_bin.tar.gz",
        ),
        windows: "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_windows-x64_bin.zip",
    },
    Jdk {
        name: "OPENJDK21",
        linux_x64: "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz",
        linux_aarch64: Some(
            "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-aarch64_bin.tar.gz",
        ),
        macos_x64: "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_macos-x64_bin.tar.gz",
        macos_aarch64: Some(
            "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_macos-aarch64_bin.tar.gz",
        ),
        windows: "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_windows-x64_bin.zip",
    },
    Jdk {
        name: "OPENJDK22",
        linux_x64: "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz",
        linux_aarch64: Some(
            "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-aarch64_bin.tar.gz",
        ),
        macos_x64: "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_macos-x64_bin.tar.gz",
        macos_aarch64: Some(
            "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_macos-aarch64_bin.tar.gz",
        ),
        windows: "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_windows-x64_bin.zip",
    },
    // graalvm jdk's
    Jdk {
        name: "GRAAL21",
        linux_x64: "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_linux-x64_bin.tar.gz",
        linux_aarch64: Some(
            "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_linux-aarch64_bin.tar.gz",
        ),
        macos_x64: "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_macos-x64_bin.tar.gz",
        macos_aarch64: Some(
            "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_macos-aarch64_bin.tar.gz",
        ),
        windows: "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_windows-x64_bin.zip",
    },
    Jdk {
        name: "GRAAL22",
        linux_x64: "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_linux-x64_bin.tar.gz",
        linux_aarch64: Some(
            "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_linux-aarch64_bin.tar.gz",
        ),
        macos_x64: "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_macos-x64_bin.tar.gz",
        macos_aarch64: Some(
            "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_macos-aarch64_bin.tar.gz",
        ),
        windows: "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_windows-x64_bin.zip",
    },
];

/// Look up a JDK by name (case-insensitive), e.g. `"zulu17"` -> `ZULU17`.
pub fn find(name: &str) -> Option<&'static Jdk> {
    let key = name.to_ascii_uppercase();
    JDKS.iter().find(|jdk| jdk.name == key)
}

#[cfg(test)]
mod tests {
    use super::{JDKS, Jdk, find};
    use std::collections::HashSet;

    #[test]
    fn jdk_names_are_unique() {
        let mut names = HashSet::new();
        for jdk in JDKS {
            assert!(names.insert(jdk.name), "duplicate jdk name: {}", jdk.name);
        }
    }

    #[test]
    fn every_jdk_ships_core_platforms() {
        for jdk in JDKS {
            assert!(!jdk.name.is_empty());
            assert!(!jdk.linux_x64.is_empty());
            assert!(!jdk.macos_x64.is_empty());
            assert!(!jdk.windows.is_empty());
        }
    }

    #[test]
    fn lookups_are_case_insensitive() {
        assert_eq!(find("zulu17").map(|jdk| jdk.name), Some("ZULU17"));
        assert_eq!(find("ZULU17").map(|jdk| jdk.name), Some("ZULU17"));
        assert_eq!(find("OpenJDK21").map(|jdk| jdk.name), Some("OPENJDK21"));
        assert!(find("not-a-jdk").is_none());
    }

    #[test]
    fn openjdk11_has_no_aarch64_builds() {
        let jdk: &Jdk = find("openjdk11").unwrap();
        assert!(jdk.linux_aarch64.is_none());
        assert!(jdk.macos_aarch64.is_none());
    }

    #[test]
    fn every_url_is_https() {
        for jdk in JDKS {
            let urls = [jdk.linux_x64, jdk.macos_x64, jdk.windows]
                .into_iter()
                .chain(jdk.linux_aarch64)
                .chain(jdk.macos_aarch64);
            for url in urls {
                assert!(
                    url.starts_with("https://"),
                    "non-https url in {}: {url}",
                    jdk.name
                );
            }
        }
    }
}
