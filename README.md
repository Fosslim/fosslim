# fosslim

FOSSlim stands for Free Open Source Software LIcense Matcher and it matches various forms of specifications of OSS licenses with SPDX ids or well-known unified license names;

It was built due te fact that other alternatives were either closed source or were just quick hack to scratch their own limited use-case. Sadly all of them miss real benchmarks with the measure the rate of false positivesm which can be very expensive error in the due diligence process;

It is not first of its kind, but it will be the first which:

* can find matches based on a license text, url text, a content of a README etc
* decreases false positives
* minimizes noise
* supports dual licensed projects
* supports all the SPDX licenses
* recognizes EULAs
* detects additional clauses in license texts
* extensively benchmarked and tested


It is still under **active development**, but it will be released as Rust library, commandline tool, simple API and RoR extension via Helix project;

#### Current alternatives

* SPDX lookup - https://github.com/bbqsrc/spdx-lookup-python
* LibrariesIO license normalizer - https://github.com/librariesio/spdx
* Google's license classifier - https://github.com/google/licenseclassifier
* Fossology - https://github.com/fossology/fossology
* LicenseFinder - https://github.com/pivotal/LicenseFinder

and many other similar hacks on Github








