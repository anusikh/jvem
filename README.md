# JVeM

![](/public/final-logo-black.png)

a simple version manager for java, node and maven
- *cross-platform support:*  consistent experience across different operating systems.
- *simple version switching:* easily switch between different java/node versions.

#### getting started:

to get started with jvem, refer to the installation instructions and basic usage guidelines below.

#### windows installation

- download the latest version from [releases](https://github.com/anusikh/jvem/releases)
- add these path to your environment variables:
	```
	C:\Users\<user>\.jvem\java\bin
	C:\Users\<user>\.jvem\node
	C:\Users\<user>\.jvem\maven
	<path-to-extracted-binary>
	```
#### macos/linux installation

- download the latest version from [releases](https://github.com/anusikh/jvem/releases)
- add the following lines to `~/.zshrc` or `~/.bashrc`
	```
	PATH="$M2_HOME/bin:$PATH"
	PATH=$PATH:$HOME/.jvem/java/bin
	PATH=$PATH:$HOME/.jvem/node/bin
	JAVA_HOME=$HOME/.jvem/java
	JDK_HOME=$HOME/.jvem/java
	alias jvem=<path-to-extracted-binary>
	```

#### usage:

run `jvem --help` to see the top-level commands, or `jvem <tool> --help` for the available actions of a specific tool:

```
jvem java <action> [version]
jvem node <action> [version]
jvem maven <action>
```

**java/node actions** (install/uninstall/usev take a version argument):

| action      | description                                                  |
| ----------- | ------------------------------------------------------------ |
| `install`   | install a version, e.g. `jvem java install zulu17`           |
| `usev`      | activate an installed version, e.g. `jvem node usev 22.11.0` |
| `uninstall` | remove an installed version, e.g. `jvem java uninstall zulu17` |
| `lsrem`     | list versions available for install                          |
| `ls`        | list locally installed versions                              |
| `current`   | show the currently active version                            |
| `deactivate`| remove the active version symlink                            |
| `clean`     | remove empty version directories left behind by failed installs |

**maven actions:** `install` and `uninstall` only.

example of listing locally installed versions:

```
$ jvem java ls
zulu17
openjdk22
```

`lsrem` lists what is available remotely. for java it prints the supported distributions:

```
$ jvem java lsrem
available versions:
zulu8
zulu11
zulu17
zulu21
zulu22
openjdk11
openjdk17
openjdk21
openjdk22
graal21
graal22
```

for node it fetches the version list from nodejs.org and groups it by major version (only majors >= 16 are supported):

```
$ jvem node lsrem
available versions:
nodejs v22: 22.0.0, 22.1.0, 22.2.0, 22.3.0, ...
nodejs v23: 23.0.0, 23.1.0, ...
```

#### exit codes:

- successful commands exit with `0`.
- failures (unknown jdk, invalid node version, download errors, missing version argument, ...) print a message to stderr and exit with `1`.

#### note for macos:

you would need to allow it from settings > under privacy and security:
<img alt="" src="https://github.com/anusikh/jvem/assets/64547846/0f37ea90-1b68-4272-a823-f8dd2390c324">
