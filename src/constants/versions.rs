pub mod constants {
    pub const LINUX_ARTIFACT: &str = "jvem.tar.gz";
    pub const WINDOWS_ARTIFACT: &str = "jvem.zip";
    pub const AVAILABLE_VERSIONS: &str =
        "ZULU8,ZULU11,ZULU17,ZULU21,OPENJDK11,OPENJDK17,OPENJDK21,GRAAL17,GRAAL21,GRAAL22";

    // zulu jdk's
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

    // openjdk jdk's
    pub const OPENJDK21_LINUX: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz";
    pub const OPENJDK21_WINDOWS: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_windows-x64_bin.zip";
    pub const OPENJDK21_MACOS_AARCH64: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_macos-aarch64_bin.tar.gz";
    pub const OPENJDK21_MACOS_X86_64: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_macos-x64_bin.tar.gz";
    pub const OPENJDK17_LINUX: &str = "https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz";
    pub const OPENJDK17_WINDOWS: &str = "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_windows-x64_bin.zip";
    pub const OPENJDK17_MACOS_AARCH64: &str = "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_macos-aarch64_bin.tar.gz";
    pub const OPENJDK17_MACOS_X86_64: &str = "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_macos-x64_bin.tar.gz";
    pub const OPENJDK11_LINUX: &str =
        "https://download.java.net/java/GA/jdk11/13/GPL/openjdk-11.0.1_linux-x64_bin.tar.gz";
    pub const OPENJDK11_WINDOWS: &str =
        "https://download.java.net/java/GA/jdk11/13/GPL/openjdk-11.0.1_windows-x64_bin.zip";
    pub const OPENJDK11_MACOS_X86_64: &str =
        "https://download.java.net/java/GA/jdk11/9/GPL/openjdk-11.0.2_osx-x64_bin.tar.gz";

    // graalvm jdk's
    pub const GRAAL22_LINUX: &str =
        "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_linux-x64_bin.tar.gz";
    pub const GRAAL22_WINDOWS: &str =
        "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_windows-x64_bin.zip";
    pub const GRAAL22_MACOS_AARCH64: &str =
        "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_macos-aarch64_bin.tar.gz";
    pub const GRAAL22_MACOS_X86_64: &str =
        "https://download.oracle.com/graalvm/22/latest/graalvm-jdk-22_macos-x64_bin.tar.gz";
    pub const GRAAL21_LINUX: &str =
        "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_linux-x64_bin.tar.gz";
    pub const GRAAL21_WINDOWS: &str =
        "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_windows-x64_bin.zip";
    pub const GRAAL21_MACOS_AARCH64: &str =
        "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_macos-aarch64_bin.tar.gz";
    pub const GRAAL21_MACOS_X86_64: &str =
        "https://download.oracle.com/graalvm/21/latest/graalvm-jdk-21_macos-x64_bin.tar.gz";
    pub const GRAAL17_LINUX: &str =
        "https://download.oracle.com/graalvm/17/latest/graalvm-jdk-17_linux-x64_bin.tar.gz";
    pub const GRAAL17_WINDOWS: &str =
        "https://download.oracle.com/graalvm/17/latest/graalvm-jdk-17_windows-x64_bin.zip";
    pub const GRAAL17_MACOS_AARCH64: &str =
        "https://download.oracle.com/graalvm/17/latest/graalvm-jdk-17_macos-aarch64_bin.tar.gz";
    pub const GRAAL17_MACOS_X86_64: &str =
        "https://download.oracle.com/graalvm/17/latest/graalvm-jdk-17_macos-x64_bin.tar.gz";

    pub fn get_constant(key: &str) -> Option<&'static str> {
        match key {
            "LINUX_ARTIFACT" => Some(LINUX_ARTIFACT),
            "WINDOWS_ARTIFACT" => Some(WINDOWS_ARTIFACT),
            "AVAILABLE_VERSIONS" => Some(AVAILABLE_VERSIONS),

            // zulu jdk's
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

            // openjdk jdk's
            "OPENJDK21_LINUX" => Some(OPENJDK21_LINUX),
            "OPENJDK21_WINDOWS" => Some(OPENJDK21_WINDOWS),
            "OPENJDK21_MACOS_AARCH64" => Some(OPENJDK21_MACOS_AARCH64),
            "OPENJDK21_MACOS_X86_64" => Some(OPENJDK21_MACOS_X86_64),
            "OPENJDK17_LINUX" => Some(OPENJDK17_LINUX),
            "OPENJDK17_WINDOWS" => Some(OPENJDK17_WINDOWS),
            "OPENJDK17_MACOS_AARCH64" => Some(OPENJDK17_MACOS_AARCH64),
            "OPENJDK17_MACOS_X86_64" => Some(OPENJDK17_MACOS_X86_64),
            "OPENJDK11_LINUX" => Some(OPENJDK11_LINUX),
            "OPENJDK11_WINDOWS" => Some(OPENJDK11_WINDOWS),
            "OPENJDK11_MACOS_X86_64" => Some(OPENJDK11_MACOS_X86_64),

            // graalvm jdk's
            "GRAAL22_LINUX" => Some(GRAAL22_LINUX),
            "GRAAL22_WINDOWS" => Some(GRAAL22_WINDOWS),
            "GRAAL22_MACOS_AARCH64" => Some(GRAAL22_MACOS_AARCH64),
            "GRAAL22_MACOS_X86_64" => Some(GRAAL22_MACOS_X86_64),
            "GRAAL21_LINUX" => Some(GRAAL21_LINUX),
            "GRAAL21_WINDOWS" => Some(GRAAL21_WINDOWS),
            "GRAAL21_MACOS_AARCH64" => Some(GRAAL21_MACOS_AARCH64),
            "GRAAL21_MACOS_X86_64" => Some(GRAAL21_MACOS_X86_64),
            "GRAAL17_LINUX" => Some(GRAAL17_LINUX),
            "GRAAL17_WINDOWS" => Some(GRAAL17_WINDOWS),
            "GRAAL17_MACOS_AARCH64" => Some(GRAAL17_MACOS_AARCH64),
            "GRAAL17_MACOS_X86_64" => Some(GRAAL17_MACOS_X86_64),

            _ => None,
        }
    }
}
