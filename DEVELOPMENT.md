next steps:
- on install, bix should initialize global config
    -> check how gitoxide does that

- ucn nodes only get the directory that they are called/built from passed as an arg. there they check for their .bix for config and others that they need
    -> bix init needs to:
        - init a local .bix
        - init a local .bix/config
        - on node runtime, check for global ownership keys and prompt fpr new ones if empty (meanign that basically the node runtime does some bix fallbacks)

## TODO 13.08.25

bix create <id>
- git init
- bix init
- clone bix repo and name node appropiately (ucn node)

/workspace
    /ucn
        /.git
    .bix
    
?

ucn init/run
(derive from env?)

bix config --gen





