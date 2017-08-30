# FOSSlim

FOSSlim stands for **F**ree **O**pen *S*ource *S*oftware **LI**cense **M**atcher and
it matches various forms of specifications of the OSS licenses with their ids, by default it would be SPDX-ids, 
but user can easily change training data;


Current design goals:

* match based on a license text, url, a content of a README etc
* decrease false positives
* support dual licensed projects
* list of licenses (default: all SPDX) is extendible
* recognizes popular EULAs
* detects additional clauses in license texts
* benchmarked and tested


It is still under **active development**, but it will be released as 

1. Rust library ( *milestone.1*, *milestone.3* )
2. RoR gem with example API ( *milestone.2* )
3. commandline tool to scan files, TBD
4. NodeJS library with example API, TBD
5. Rust Microservice, TBD


#### Usage

```rust

//TODO: after first release
```


#### Current alternatives

here are some of alternatives you could use already now:

* SPDX lookup - https://github.com/bbqsrc/spdx-lookup-python
* LibrariesIO license normalizer - https://github.com/librariesio/spdx
* **Google's license classifier** - https://github.com/google/licenseclassifier
* **Fossology** - https://github.com/fossology/fossology
* LicenseFinder - https://github.com/pivotal/LicenseFinder
