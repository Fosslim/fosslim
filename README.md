# FOSSlim

![](https://img.shields.io/crates/v/fosslim.svg)

FOSSlim stands for **F**ree **O**pen *S*ource *S*oftware **LI**cense **M**atcher and
it matches the text of the OSS license with SPDX id, but user can easily change & update training data
with additional EULAs and license text;

It is designed to be modular and to provide many low-level high-speed utilities which libraries written in high-level
languages like Ruby & Javascript could benefit;
Which means you could take advantage of various models implemented here, but they alone are not enough to provide a response
with high-confidence. This task is left for the RubyGem & NPM packages, which are cleaning up a raw-text 
and combining results from multiple models to increase the confidence of the match result;

It is still under **active development**, but it will be released as 

1. ~~Rust library ( *milestone.1*, *milestone.3* )~~
2. ~~RoR gem with example API ( *milestone.2* )~~ - [LicenseMatcher gem](https://rubygems.org/gems/license_matcher)
3. sample RoR application using the GEM - Fosslim.com

... TBD = release time unknown: priority depends on interests from community
4. NodeJS library with example AWS lambda function, TBD
5. Rust Microservice, TBD
6. commandline tool to scan files, TBD

#### Models

* **NaiveTF** - uses simple WordBag model and ranks results by [Jaccard similarity](https://en.wikipedia.org/wiki/Jaccard_index)
* **FingerNgram** - splits text into overlapping [Ngrams](https://en.wikipedia.org/wiki/N-gram) and hashes selected NGrams for fingerprint;

... in near future
*  TF/IDF models with Cosine similarity
* Okapi25 model
* Winnowing model
* Simple probabilistic ML models ~ Naive Bayes, HMM, ...?

#### Usage

```rust
use fosslim::index;
use fosslim::document::Document;
use fosslim::naive_tf; // Simple wordbag model with Jaccard similarity
...
let idx_file_path = "data/index.msgpack"; // it is pre-built index from SPDX data, includes ~300 licenses
let mit_txt = r#"
Permission is hereby granted, free of charge, to any person obtaining a copy of this software \
and associated documentation files (the "Software"), to deal in the Software without restriction,\
including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,\
and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,\
subject to the following conditions:\
"#;

let doc1 = Document::new(0, "mit".to_string(), mit_txt.to_string());


// matching document with SPDX label
if let Ok(idx) = index::load(idx_file_path) {
    let mdl = naive_tf::from_index(&idx);
    
    mdl::match_document(&doc1);
}
...
```

check `tests` folder for more usage examples;

And yes, you can build your own index with `index::build_from_path()` function; you just have to use same file structure
the JSON files in the `data/licenses` folder;

#### Current alternatives

here are some of alternatives you could use already now:

* SPDX lookup - https://github.com/bbqsrc/spdx-lookup-python
* LibrariesIO license normalizer - https://github.com/librariesio/spdx
* **Google's license classifier** - https://github.com/google/licenseclassifier
* **Fossology** - https://github.com/fossology/fossology
* LicenseFinder - https://github.com/pivotal/LicenseFinder
