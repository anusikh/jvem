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

- just run the  `jvem --help` command

#### note for macos:

you would need to allow it from settings > under privacy and security:
<img alt="" src="https://github.com/anusikh/jvem/assets/64547846/0f37ea90-1b68-4272-a823-f8dd2390c324">
