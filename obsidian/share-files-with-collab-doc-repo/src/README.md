---
key:"value"
---

## Share files with collab doc repo obsidian automation script

This script allows to share files with a collab doc repo by looking for a key in the file frontmatter section and then copying the file to the collab doc repo, commit a new version and push the changes.

It can be used to create a scheduled task that will automatically share files that are not previousle be shared, and to share files that are already shared but have been updated in the source repository since the last time the script was run.

This file also serves as a test file to see if the script is working.

## Usage

```bash
$ share-files-with-collab-doc-repo --source-dir /path/to/source/repo --dest-dir /path/to/collab/doc/repo --key key --branch branch-name
```

### With docker container
    
```bash
$ docker run -it --rm -v /path/to/source/repo:/source -v /path/to/collab/doc/repo:/collab share-files-with-collab-doc-repo --source-dir /source --dest-dir /collab --key key --branch branch-name
```

