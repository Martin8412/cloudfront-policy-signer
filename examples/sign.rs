use cloudfront_policy_signer;

fn main() {
    // Replace me with actual URI
    let resource = "https://example.cloudfront.net/flowerpot.png";
    // Replace me with actual desired expiry time
    let expiry = 1579532331;
    // Replace the key
    let certificate_location = "examples/key.pem";
    // Replace with Key-Pair-Id from AWS console
    let key_pair_id = "APKAIEXAMPLE";

    let signature = cloudfront_policy_signer::create_canned_policy_signature(resource, expiry, certificate_location).unwrap();

    println!("Signed URL is {}", format!("{}?Expires={}&Signature={}&Key-Pair-Id={}", resource, expiry, signature, key_pair_id));
}