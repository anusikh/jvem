# Java Version Manager (JVeM)

![JVeM](/public/final-logo-black.png)

Java Version Manager (JVeM) is a cross-platform tool designed to simplify the management of multiple Java versions on your system. Whether you're a developer working on diverse projects or an enthusiast exploring different Java applications, JVeM makes it easy to switch between and manage various Java versions seamlessly.

## Table of Contents

1. [Features](#features)
2. [Supported Platforms](#supported-platforms)
3. [Getting Started](#getting-started)
   - [Windows Installation](#windows-installation)
   - [macOS Installation](#macos-installation)
   - [Linux Installation](#linux-installation)
4. [Usage](#usage)
   - [Commands and Descriptions](#commands-and-descriptions)
5. [Future Scope](#future-scope)

## Features:

- **Cross-Platform Support:** JVeM is compatible with macOS, Linux, and Windows, providing a consistent experience across different operating systems.
- **Simple Version Switching:** Easily switch between different Java versions with a single command, streamlining your development workflow.
- **Effortless Installation:** JVeM offers a straightforward installation process on supported platforms, ensuring quick setup without unnecessary complications.
- **Customizable Configuration:** Tailor JVeM to your specific needs by configuring environment variables and settings to suit your Java development environment.

## Supported Platforms:

- Windows
- Linux
- macOS

## Getting Started:

To get started with JVeM, refer to the installation instructions and basic usage guidelines below.

#### Windows Installation

- Download the latest `jvem.zip` file from [Releases](https://github.com/anusikh/jvem/releases)
- Extract this file here.
- Add this path to your environment variables: `C:\Users\<user>\.jvem\java\bin`
- Add the path of the extracted folder to environment variables.

#### macOS Installation

- Download the latest `jvem_macos.tar.gz` file from [Releases](https://github.com/anusikh/jvem/releases)
- Extract the tarball file.
- Add this line to your zshrc/bashrc file: `export JAVA_HOME=$HOME/.jvem/java` and source it.
- Add an alias to use jvem easily. `alias jvem=~/Downloads/jvem`

#### Linux Installation

- Download the latest `jvem_linux.tar.gz` file from [Releases](https://github.com/anusikh/jvem/releases)
- Extract the tarball file.
- Add this line to your zshrc/bashrc file: `PATH=$PATH:$HOME/.jvem/java/bin`
- Also add this line `JAVA_HOME=$HOME/.jvem/java` and then source the rc file
- Add an alias to use jvem easily. `alias jvem=~/Downloads/jvem`

## Usage:

Once installed, use the following commands to manage your Java versions.
1. Check all versions of java supported by JVeM:
```
jvem lsrem
```
2. Install a new version on your system:
```
jvem install zulu17
```
3. Start using the java version installed
```
jvem usev zulu17
```
4. Check if java got installed:

   If java versions < 8:
`java -version `
   else:
`java --version`
5. Uninstall java
`jvem uninstall zulu17`


## Commands and Descriptions

| Command                        | Description                                    |
| ------------------------------ | ---------------------------------------------- |
| `jvem lsrem`                   | List all JDK versions available for install.   |
| `jvem ls`                      | List locally installed JDK versions.           |
| `jvem install zulu21`          | Install Zulu JDK version 21.                   |
| `jvem usev zulu17`             | Use a specific JDK version after installation. |
| `jvem current`                 | Find the currently active JDK version.         |
| `jvem deactivate`              | Deactivate the currently active JDK.           |
| `jvem uninstall zulu17`        | Uninstall the specified JDK version.           |
| `jvem help`                    | Display help information.                      |
| `jvem clean`                   | Clean empty folders in the .jvem directory.    |
| `jvem --V` or `jvem --version` | Display the version of JVeM.                   |

## Future Scope:

JVeM is an evolving project, and future updates may include:

- Addition of more JDKs
- Addition of support for new platforms.
- Integration of test cases to enhance stability.
- Bug fixes and improvements based on user feedback.

Explore the full capabilities of JVeM to enhance your Java development experience!



### Note for MacOs:
This might show an error when trying to use it for the first time, you would need to allow it from settings > under privacy and security:
<img width="710" alt="Screenshot 2024-03-09 at 2 14 49 AM" src="https://github.com/anusikh/jvem/assets/64547846/0f37ea90-1b68-4272-a823-f8dd2390c324">
