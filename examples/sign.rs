/*
MIT License

Copyright (c) 2020 Martin Karlsen Jensen

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use cloudfront_policy_signer;
use cloudfront_policy_signer::CloudFrontCannedPolicySigner;

fn main() {
    // Replace me with actual URI
    let resource = "https://example.cloudfront.net/flowerpot.png";
    // Replace me with actual desired expiry time
    let expiry = 2579532331;
    // Replace the key
    let certificate_location = "examples/key.pem";
    // Replace with Key-Pair-Id from AWS console
    let key_pair_id = "APKAIEXAMPLE";

    let signature = cloudfront_policy_signer::create_canned_policy_signature(
        resource,
        expiry,
        certificate_location,
    )
    .unwrap();
    let signed_url = format!(
        "{}?Expires={}&Signature={}&Key-Pair-Id={}",
        resource, expiry, signature, key_pair_id
    );

    println!("Signed URL is {}", signed_url);

    let signed_url_struct = CloudFrontCannedPolicySigner::new("examples/key.pem")
        .unwrap()
        .create_canned_policy_signature_url(resource, expiry, key_pair_id)
        .unwrap();

    assert_eq!(signed_url, signed_url_struct);
}
