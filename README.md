## jvem (WIP)

### about

a java version manager for windows and linux

```
# or zulu17 or zulu11 or zulu8
jvem install zulu21

# list all jdk's available for install
jvem lsrem

# locally installed
jvem ls

# use a specific jdk version after install
jvem usev zulu17

# find the current jdk being used
jvem current

# deactivate current jdk
jvem deactivate

# uninstall a jdk
jvem uninstall zulu17

# help
jvem help

# clean empty folders in .jvem directory
jvem clean

# version
jvem --V/--version
```

### installation

for windows

- download the .exe file from the [releases](https://github.com/anusikh/jvem/releases)
- add this to path env variable: `C:\Users\<user>\.jvem\java\bin)`
- also add the path to the downloaded .exe file in the path env variable: `C:\Users\<user>\Desktop\jvm.exe)`
- replace user with the appropriate user folder name

for linux

- download the binary from [releases](https://github.com/anusikh/jvem/releases)
- open .bashrc/.zshrc and paste this: `PATH=$PATH:$HOME/.jvem/java/bin`
- also add this: `alias jvem=<path-to-jvem-binary>`
- open the terminal and run: `source ~/.bashrc` or `source ~/.zshrc`

### todo

- mac support
- test cases
- installation script
  - linux (make sure you add export PATH=$PATH:/home/anusikh/.jvem/java/bin in .bashrc and source it)
  - windows (make sure you add this to Path env: C:\Users\anusi\.jvem\java\bin)
- bug fixes
