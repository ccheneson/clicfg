# clicfg - a simple command line tool to manage config ( or any ) files located in AWS S3

I often need to change configuration files for several environments when deploying features and 
I always find such a pain to use AWS Web Console to browse bucket/folders to download a file, modify it, bump its version and re-upload it with the proper encryption/key. 

This cli is used to manage application configuration files stored in S3.

Configs are located 
* at a path in S3 like `bucket`/`project`/`environment`/`file`
* locally at `$HOME`/conf/`project`/`environment`/`file`

`file` could be like `1.5.12.conf` or `api-back.1.2.2.conf`

This is also my first application/tool in Rust

# How to use it ?

* List in S3 the existing conf for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging ls`

* Locally List the existing conf for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging lslo`

* Get a conf for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging get 1.20.9.conf`

* Put a conf for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging put 2.0.0.conf`

* Bump a conf to a new version for a particular project/environment (e.g. `api/staging`)

`$> clicfg api staging bump 2.0.0 2.1.0`

The above command will download `2.0.0.conf`, rename it the to version `2.1.0.conf`, and re-upload it to the `api/staging`

`$> clicfg api staging diff 2.0.0.conf 2.1.0.conf`

The above command highlights the difference between the file `2.0.0.conf` and the file `2.1.0.conf`

`$> clicfg api staging edit 2.1.0.conf`

The above command downloads the file `2.1.0.conf`, opens the file with the editor sets in `PMCFG_EDITOR` environment variable (default set to `vi`). When saving and closing `vi` (`:wq!`), it pushes the file back to the correct S3 path. 

It doesn't work with GUI-based editors like `gedit`

