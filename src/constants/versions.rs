pub mod constants {
    pub const LINUX_ARTIFACT: &str = "jvem.tar.gz";
    pub const WINDOWS_ARTIFACT: &str = "jvem.zip";
    pub const AVAILABLE_VERSIONS: &str = "ZULU8,ZULU11,ZULU17,ZULU21";
    pub const ZULU21_LINUX: &str =
        "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-linux_x64.tar.gz";
    pub const ZULU21_WINDOWS: &str = "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-win_x64.zip?_gl=1*14dmzoc*_ga*MTg0NTAyMjU0LjE3MDg3MDUwMzU.*_ga_42DEGWGYD5*MTcwODcwNTAzNS4xLjEuMTcwODcwNTExMi40NC4wLjA.";
    pub const ZULU21_MACOS_AARCH64: &str = "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-macosx_aarch64.tar.gz?_gl=1*bqdjc4*_ga*MTU1MDc4NTgxMS4xNzA5NzU3MDc2*_ga_42DEGWGYD5*MTcwOTc1NzA3NS4xLjEuMTcwOTc1NzY1NC4zOC4wLjA.";
    pub const ZULU21_MACOS_X86_64: &str = "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-macosx_x64.tar.gz?_gl=1*1ku96gu*_ga*MTU1MDc4NTgxMS4xNzA5NzU3MDc2*_ga_42DEGWGYD5*MTcwOTc1NzA3NS4xLjEuMTcwOTc1NzU5OS4yOC4wLjA.";
    pub const ZULU17_LINUX: &str =
        "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-linux_x64.tar.gz";
    pub const ZULU17_WINDOWS: &str = "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-win_x64.zip?_gl=1*1nexba0*_ga*MTg0NTAyMjU0LjE3MDg3MDUwMzU.*_ga_42DEGWGYD5*MTcwODcwNTAzNS4xLjEuMTcwODcwNTE0Ny45LjAuMA..";
    pub const ZULU17_MACOS_AARCH64: &str = "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-macosx_aarch64.tar.gz?_gl=1*ft6eo*_ga*MTU1MDc4NTgxMS4xNzA5NzU3MDc2*_ga_42DEGWGYD5*MTcwOTc1NzA3NS4xLjEuMTcwOTc1Nzg1Ni4zOC4wLjA.";
    pub const ZULU17_MACOS_X86_64: &str = "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-macosx_x64.tar.gz?_gl=1*s7p9be*_ga*MTU1MDc4NTgxMS4xNzA5NzU3MDc2*_ga_42DEGWGYD5*MTcwOTc1NzA3NS4xLjEuMTcwOTc1Nzg1Ni4zOC4wLjA.";
    pub const ZULU11_LINUX: &str =
        "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-linux_x64.tar.gz";
    pub const ZULU11_WINDOWS: &str = "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-win_x64.zip?_gl=1*2j7qzc*_ga*MTg0NTAyMjU0LjE3MDg3MDUwMzU.*_ga_42DEGWGYD5*MTcwODcwNTAzNS4xLjEuMTcwODcwNTIwNS4yNS4wLjA.";
    pub const ZULU11_MACOS_AARCH64: &str = "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-macosx_aarch64.tar.gz?_gl=1*1qf5npm*_ga*MTU1MDc4NTgxMS4xNzA5NzU3MDc2*_ga_42DEGWGYD5*MTcwOTc1NzA3NS4xLjEuMTcwOTc1Nzk5Ny40MC4wLjA.";
    pub const ZULU11_MACOS_X86_64: &str = "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-macosx_x64.tar.gz?_gl=1*10u3b0w*_ga*MTU1MDc4NTgxMS4xNzA5NzU3MDc2*_ga_42DEGWGYD5*MTcwOTc1NzA3NS4xLjEuMTcwOTc1Nzk5Ny40MC4wLjA.";
    pub const ZULU8_LINUX: &str =
        "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-linux_i686.tar.gz";
    pub const ZULU8_WINDOWS: &str ="https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-win_x64.zip?_gl=1*i45ppb*_ga*MTg0NTAyMjU0LjE3MDg3MDUwMzU.*_ga_42DEGWGYD5*MTcwODcwNTAzNS4xLjEuMTcwODcwNTIxMy4xNy4wLjA.";
    pub const ZULU8_MACOS_AARCH64: &str = "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-macosx_aarch64.tar.gz?_gl=1*18knssl*_ga*MTU1MDc4NTgxMS4xNzA5NzU3MDc2*_ga_42DEGWGYD5*MTcwOTc1NzA3NS4xLjEuMTcwOTc1ODA1MS41MC4wLjA.";
    pub const ZULU8_MACOS_X86_64: &str = "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-macosx_x64.tar.gz?_gl=1*18knssl*_ga*MTU1MDc4NTgxMS4xNzA5NzU3MDc2*_ga_42DEGWGYD5*MTcwOTc1NzA3NS4xLjEuMTcwOTc1ODA1MS41MC4wLjA.";
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
            "ZULU21_MACOS_AARCH64" => Some(ZULU21_MACOS_AARCH64),
            "ZULU21_MACOS_X86_64" => Some(ZULU21_MACOS_X86_64),
            "ZULU17_MACOS_AARCH64" => Some(ZULU17_MACOS_AARCH64),
            "ZULU17_MACOS_X86_64" => Some(ZULU17_MACOS_X86_64),
            "ZULU11_MACOS_AARCH64" => Some(ZULU11_MACOS_AARCH64),
            "ZULU11_MACOS_X86_64" => Some(ZULU11_MACOS_X86_64),
            "ZULU8_MACOS_AARCH64" => Some(ZULU8_MACOS_AARCH64),
            "ZULU8_MACOS_X86_64" => Some(ZULU8_MACOS_X86_64),
            _ => None,
        }
    }
}
