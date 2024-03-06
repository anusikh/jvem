pub mod constants {
    pub const LINUX_ARTIFACT: &str = "jvem.tar.gz";
    pub const WINDOWS_ARTIFACT: &str = "jvem.zip";
    pub const AVAILABLE_VERSIONS: &str = "ZULU8,ZULU11,ZULU17,ZULU21";
    pub const ZULU21_LINUX: &str =
        "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-linux_x64.tar.gz";
    pub const ZULU21_WINDOWS: &str = "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-win_x64.zip?_gl=1*14dmzoc*_ga*MTg0NTAyMjU0LjE3MDg3MDUwMzU.*_ga_42DEGWGYD5*MTcwODcwNTAzNS4xLjEuMTcwODcwNTExMi40NC4wLjA.";
    // ZULU21_MACOS

    pub const ZULU17_LINUX: &str =
        "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-linux_x64.tar.gz";
    pub const ZULU17_WINDOWS: &str = "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-win_x64.zip?_gl=1*1nexba0*_ga*MTg0NTAyMjU0LjE3MDg3MDUwMzU.*_ga_42DEGWGYD5*MTcwODcwNTAzNS4xLjEuMTcwODcwNTE0Ny45LjAuMA..";
    // ZULU17_MACOS

    pub const ZULU11_LINUX: &str =
        "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-linux_x64.tar.gz";
    pub const ZULU11_WINDOWS: &str = "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-win_x64.zip?_gl=1*2j7qzc*_ga*MTg0NTAyMjU0LjE3MDg3MDUwMzU.*_ga_42DEGWGYD5*MTcwODcwNTAzNS4xLjEuMTcwODcwNTIwNS4yNS4wLjA.";
    // ZULU11_MACOS

    pub const ZULU8_LINUX: &str =
        "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-linux_i686.tar.gz";
    pub const ZULU8_WINDOWS: &str ="https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-win_x64.zip?_gl=1*i45ppb*_ga*MTg0NTAyMjU0LjE3MDg3MDUwMzU.*_ga_42DEGWGYD5*MTcwODcwNTAzNS4xLjEuMTcwODcwNTIxMy4xNy4wLjA.";
    // ZULU8_MACOS

    pub fn get_constant(key: &str) -> Option<&'static str> {
        match key {
            "LINUX_ARTIFACT" => Some(LINUX_ARTIFACT),
            "WINDOWS_ARTIFACT" => Some(WINDOWS_ARTIFACT),
            "AVAILABLE_VERSIONS" => Some(AVAILABLE_VERSIONS),
            "ZULU21_LINUX" => Some(ZULU21_LINUX),
            "ZULU21_WINDOWS" => Some(ZULU21_WINDOWS),
            "ZULU17_LINUX" => Some(ZULU17_LINUX),
            "ZULU17_WINDOWS" => Some(ZULU17_WINDOWS),
            "ZULU11_LINUX" => Some(ZULU11_LINUX),
            "ZULU11_WINDOWS" => Some(ZULU11_WINDOWS),
            "ZULU8_LINUX" => Some(ZULU8_LINUX),
            "ZULU8_WINDOWS" => Some(ZULU8_WINDOWS),
            _ => None,
        }
    }
}
