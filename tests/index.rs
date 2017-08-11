extern crate fosslim;

#[test]
fn test_index_example1(){
    let test_doc = r#"{
        "isDeprecatedLicenseId": false,
        "licenseText": "Copyright (C) 2006 by Rob Landley \u003crob@landley.net\u003e\n\nPermission to use, copy, modify, s hereby granted.\n",
        "standardLicenseTemplate": "Copyright (C) 2006 by Rob Landley \u003crob@landley.net\u003e\n\nPermission to use, copy, modify,ee is hereby granted.\n ",
        "name": "BSD Zero Clause License",
        "licenseId": "0BSD",
        "standardLicenseHeader": "",
        "seeAlso": [
            "http://landley.net/toybox/license.html"
        ],
        "isOsiApproved": true
    }"#;

    let res = fosslim::index::build_from_json(test_doc);
    assert!(res.is_ok());
}