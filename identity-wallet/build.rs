fn main() {
    // Re-run the build script if the .env file changes
    println!("cargo:rerun-if-changed=.env");

    let email_verification_service_host = dotenvy::var("EMAIL_VERIFICATION_SERVICE_HOST")
        .unwrap_or_else(|_| "https://email-verification.impierce.com".to_string());
    let email_verification_service_api_key =
        dotenvy::var("EMAIL_VERIFICATION_SERVICE_API_KEY").unwrap_or_else(|_| "".to_string());

    println!(
        "cargo:rustc-env=EMAIL_VERIFICATION_SERVICE_HOST={}",
        email_verification_service_host
    );
    println!(
        "cargo:rustc-env=EMAIL_VERIFICATION_SERVICE_API_KEY={}",
        email_verification_service_api_key
    );
}
