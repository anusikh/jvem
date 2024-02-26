### jvem (WIP)

a java version manager

- cargo run -- install zulu21
- cargo run -- lsrem
- cargo run -- ls
- cargo run -- usev zulu17
- cargo run -- current
- cargo run -- deactivate
- cargo run -- uninstall zulu17

TODO:
- ~~lsrem~~
- ~~install~~
    - ~~linux~~ 
    - ~~windows~~
- ~~ls (list all locally installed versions)~~
- ~~improve command line outputs~~
- ~~usev (will be done using symlinks)~~
    - ~~linux~~ 
    - ~~windows~~ 
- ~~deactivate~~
    - ~~linux~~ 
    - ~~windows~~ 
- ~~current~~
- ~~uninstall~~
    - ~~linux~~ 
    - ~~windows~~ 
- ~~help~~
- test cases
- signal interrupt handling
    - linux
    - windows
- publishing pipeline
- installation script 
    - linux (make sure you add export PATH=$PATH:/home/anusikh/.jvem/java/bin in .bashrc and source it)
    - windows (make sure you add this to Path env: C:\Users\anusi\.jvem\java\bin)
- bug fixes