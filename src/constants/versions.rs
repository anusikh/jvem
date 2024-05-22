pub mod constants {
    pub const LINUX_ARTIFACT: &str = "jvem.tar.gz";
    pub const WINDOWS_ARTIFACT: &str = "jvem.zip";
    pub const AVAILABLE_VERSIONS: &str =
        "ZULU8,ZULU11,ZULU17,ZULU21,ZULU22,OPENJDK11,OPENJDK17,OPENJDK21,OPENJDK22,GRAAL17,GRAAL21,GRAAL22";

    // zulu jdk's
    pub const ZULU22_LINUX_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-linux_aarch64.tar.gz";
    pub const ZULU22_LINUX_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-linux_x64.tar.gz";
    pub const ZULU22_WINDOWS: &str =
        "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-win_x64.zip";
    pub const ZULU22_MACOS_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-macosx_aarch64.tar.gz";
    pub const ZULU22_MACOS_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu22.30.13-ca-jdk22.0.1-macosx_x64.tar.gz";

    pub const ZULU21_LINUX_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-linux_x64.tar.gz";
    pub const ZULU21_LINUX_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-linux_aarch64.tar.gz";
    pub const ZULU21_WINDOWS: &str =
        "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-win_x64.zip";
    pub const ZULU21_MACOS_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-macosx_aarch64.tar.gz";
    pub const ZULU21_MACOS_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu21.32.17-ca-jdk21.0.2-macosx_x64.tar.gz";

    pub const ZULU17_LINUX_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-linux_x64.tar.gz";
    pub const ZULU17_LINUX_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-linux_aarch64.tar.gz";
    pub const ZULU17_WINDOWS: &str =
        "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-win_x64.zip";
    pub const ZULU17_MACOS_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-macosx_aarch64.tar.gz";
    pub const ZULU17_MACOS_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu17.48.15-ca-jdk17.0.10-macosx_x64.tar.gz";

    pub const ZULU11_LINUX_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-linux_x64.tar.gz";
    pub const ZULU11_LINUX_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-linux_aarch64.tar.gz";
    pub const ZULU11_WINDOWS: &str =
        "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-win_x64.zip";
    pub const ZULU11_MACOS_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-macosx_aarch64.tar.gz";
    pub const ZULU11_MACOS_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu11.70.15-ca-jdk11.0.22-macosx_x64.tar.gz";

    pub const ZULU8_LINUX_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu8.78.0.19-ca-jdk8.0.412-linux_x64.zip";
    pub const ZULU8_LINUX_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu8.78.0.19-ca-jdk8.0.412-linux_aarch64.tar.gz";
    pub const ZULU8_WINDOWS: &str =
        "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-win_x64.zip";
    pub const ZULU8_MACOS_AARCH64: &str =
        "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-macosx_aarch64.tar.gz";
    pub const ZULU8_MACOS_X86_64: &str =
        "https://cdn.azul.com/zulu/bin/zulu8.76.0.17-ca-jdk8.0.402-macosx_x64.tar.gz";

    // openjdk jdk's
    pub const OPENJDK22_LINUX_X86_64: &str = "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-x64_bin.tar.gz";
    pub const OPENJDK22_LINUX_AARCH64: &str = "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_linux-aarch64_bin.tar.gz";
    pub const OPENJDK22_WINDOWS: &str = "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_windows-x64_bin.zip";
    pub const OPENJDK22_MACOS_AARCH64: &str = "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_macos-aarch64_bin.tar.gz";
    pub const OPENJDK22_MACOS_X86_64: &str = "https://download.java.net/java/GA/jdk22.0.1/c7ec1332f7bb44aeba2eb341ae18aca4/8/GPL/openjdk-22.0.1_macos-x64_bin.tar.gz";

    pub const OPENJDK21_LINUX_X86_64: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-x64_bin.tar.gz";
    pub const OPENJDK21_LINUX_AARCH64: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_linux-aarch64_bin.tar.gz";
    pub const OPENJDK21_WINDOWS: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_windows-x64_bin.zip";
    pub const OPENJDK21_MACOS_AARCH64: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_macos-aarch64_bin.tar.gz";
    pub const OPENJDK21_MACOS_X86_64: &str = "https://download.java.net/java/GA/jdk21.0.2/f2283984656d49d69e91c558476027ac/13/GPL/openjdk-21.0.2_macos-x64_bin.tar.gz";

    pub const OPENJDK17_LINUX_X86_64: &str = "https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-x64_bin.tar.gz";
    pub const OPENJDK17_LINUX_AARCH64: &str = "https://download.java.net/java/GA/jdk17.0.2/dfd4a8d0985749f896bed50d7138ee7f/8/GPL/openjdk-17.0.2_linux-aarch64_bin.tar.gz";
    pub const OPENJDK17_WINDOWS: &str = "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_windows-x64_bin.zip";
    pub const OPENJDK17_MACOS_AARCH64: &str = "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_macos-aarch64_bin.tar.gz";
    pub const OPENJDK17_MACOS_X86_64: &str = "https://download.java.net/java/GA/jdk17.0.1/2a2082e5a09d4267845be086888add4f/12/GPL/openjdk-17.0.1_macos-x64_bin.tar.gz";

    pub const OPENJDK11_LINUX_X86_64: &str =
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
            "ZULU22_LINUX_X86_64" => Some(ZULU22_LINUX_X86_64),
            "ZULU22_LINUX_AARCH64" => Some(ZULU22_LINUX_AARCH64),
            "ZULU22_WINDOWS" => Some(ZULU22_WINDOWS),
            "ZULU22_MACOS_AARCH64" => Some(ZULU22_MACOS_AARCH64),
            "ZULU22_MACOS_X86_64" => Some(ZULU22_MACOS_X86_64),

            "ZULU21_LINUX_X86_64" => Some(ZULU21_LINUX_X86_64),
            "ZULU21_LINUX_AARCH64" => Some(ZULU21_LINUX_AARCH64),
            "ZULU21_WINDOWS" => Some(ZULU21_WINDOWS),
            "ZULU21_MACOS_AARCH64" => Some(ZULU21_MACOS_AARCH64),
            "ZULU21_MACOS_X86_64" => Some(ZULU21_MACOS_X86_64),

            "ZULU17_LINUX_X86_64" => Some(ZULU17_LINUX_X86_64),
            "ZULU17_LINUX_AARCH64" => Some(ZULU17_LINUX_AARCH64),
            "ZULU17_WINDOWS" => Some(ZULU17_WINDOWS),
            "ZULU17_MACOS_AARCH64" => Some(ZULU17_MACOS_AARCH64),
            "ZULU17_MACOS_X86_64" => Some(ZULU17_MACOS_X86_64),

            "ZULU11_LINUX_X86_64" => Some(ZULU11_LINUX_X86_64),
            "ZULU11_LINUX_AARCH64" => Some(ZULU11_LINUX_AARCH64),
            "ZULU11_WINDOWS" => Some(ZULU11_WINDOWS),
            "ZULU11_MACOS_AARCH64" => Some(ZULU11_MACOS_AARCH64),
            "ZULU11_MACOS_X86_64" => Some(ZULU11_MACOS_X86_64),

            "ZULU8_LINUX_X86_64" => Some(ZULU8_LINUX_X86_64),
            "ZULU8_LINUX_AARCH64" => Some(ZULU8_LINUX_AARCH64),
            "ZULU8_WINDOWS" => Some(ZULU8_WINDOWS),
            "ZULU8_MACOS_AARCH64" => Some(ZULU8_MACOS_AARCH64),
            "ZULU8_MACOS_X86_64" => Some(ZULU8_MACOS_X86_64),

            // openjdk jdk's
            "OPENJDK22_LINUX_X86_64" => Some(OPENJDK22_LINUX_X86_64),
            "OPENJDK22_LINUX_AARCH64" => Some(OPENJDK22_LINUX_AARCH64),
            "OPENJDK22_WINDOWS" => Some(OPENJDK22_WINDOWS),
            "OPENJDK22_MACOS_AARCH64" => Some(OPENJDK22_MACOS_AARCH64),
            "OPENJDK22_MACOS_X86_64" => Some(OPENJDK22_MACOS_X86_64),

            "OPENJDK21_LINUX_X86_64" => Some(OPENJDK21_LINUX_X86_64),
            "OPENJDK21_LINUX_AARCH64" => Some(OPENJDK21_LINUX_AARCH64),
            "OPENJDK21_WINDOWS" => Some(OPENJDK21_WINDOWS),
            "OPENJDK21_MACOS_AARCH64" => Some(OPENJDK21_MACOS_AARCH64),
            "OPENJDK21_MACOS_X86_64" => Some(OPENJDK21_MACOS_X86_64),

            "OPENJDK17_LINUX_X86_64" => Some(OPENJDK17_LINUX_X86_64),
            "OPENJDK17_LINUX_AARCH64" => Some(OPENJDK21_LINUX_AARCH64),
            "OPENJDK17_WINDOWS" => Some(OPENJDK17_WINDOWS),
            "OPENJDK17_MACOS_AARCH64" => Some(OPENJDK17_MACOS_AARCH64),
            "OPENJDK17_MACOS_X86_64" => Some(OPENJDK17_MACOS_X86_64),

            "OPENJDK11_LINUX_X86_64" => Some(OPENJDK11_LINUX_X86_64),
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
