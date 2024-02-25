### jvem (WIP)

a java version manager

- cargo run -- install zulu21
- cargo run -- lsrem
- cargo run -- usev zulu 17

TODO:
- ~~lsrem~~
- ~~install~~
    - ~~linux~~ 
    - ~~windows~~
- ~~ls (list all locally installed versions)~~
- ~~improve command line outputs~~
- usev
    - linux (create a new rc file and set env variables, include it in bashrc file, source it)
    - ~~windows~~ (create a powershell profile, include it in the main profile, set aliases)
- installation script (make sure this is added to .jvemrc and jvemprofile.ps1)
    - linux
    - windows
- deactivate
- uninstall
- help
- test cases
- signal interrupt handling
    - linux
    - windows
- publishing pipeline
