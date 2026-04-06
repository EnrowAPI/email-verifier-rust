# Email Verifier - Rust Library

[![crates.io](https://img.shields.io/crates/v/email-verifier.svg)](https://crates.io/crates/email-verifier)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](https://opensource.org/licenses/MIT)
[![GitHub stars](https://img.shields.io/github/stars/EnrowAPI/email-verifier-rust)](https://github.com/EnrowAPI/email-verifier-rust)
[![Last commit](https://img.shields.io/github/last-commit/EnrowAPI/email-verifier-rust)](https://github.com/EnrowAPI/email-verifier-rust/commits)

Verify email addresses in real time. Check deliverability, detect disposable and catch-all domains, and clean your email lists before sending.

Powered by [Enrow](https://enrow.io) -- real-time SMTP-level verification with high accuracy on catch-all domains.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
email-verifier = "1.0"
tokio = { version = "1", features = ["full"] }
```

## Simple Usage

```rust
use email_verifier::{verify_email, get_verification_result, VerifyEmailParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let params = VerifyEmailParams {
        email: "tcook@apple.com".into(),
        webhook: None,
    };

    let verification = verify_email("your_api_key", &params).await?;
    let result = get_verification_result("your_api_key", &verification.id).await?;

    println!("{}", result.email.unwrap_or_default());         // tcook@apple.com
    println!("{}", result.qualification.unwrap_or_default()); // valid

    Ok(())
}
```

`verify_email` returns a verification ID. The verification runs asynchronously -- call `get_verification_result` to retrieve the result once it's ready. You can also pass a `webhook` URL to get notified automatically.

## Bulk verification

```rust
use email_verifier::{verify_emails, get_verification_results, VerifyEmailsParams, BulkVerification};

let params = VerifyEmailsParams {
    verifications: vec![
        BulkVerification {
            email: "tcook@apple.com".into(),
            custom: None,
        },
        BulkVerification {
            email: "satya@microsoft.com".into(),
            custom: None,
        },
        BulkVerification {
            email: "jensen@nvidia.com".into(),
            custom: None,
        },
    ],
    webhook: None,
};

let batch = verify_emails("your_api_key", &params).await?;
// batch.batch_id, batch.total, batch.status

let results = get_verification_results("your_api_key", &batch.batch_id).await?;
// results.results -- Vec<VerificationResult>
```

Up to 5,000 verifications per batch. Pass a `webhook` URL to get notified when the batch completes.

## Error handling

```rust
match verify_email("bad_key", &params).await {
    Ok(result) => println!("Verified: {:?}", result),
    Err(e) => {
        // e contains the API error description
        // Common errors:
        // - "Invalid or missing API key" (401)
        // - "Your credit balance is insufficient." (402)
        // - "Rate limit exceeded" (429)
        eprintln!("Error: {}", e);
    }
}
```

## Getting an API key

Register at [app.enrow.io](https://app.enrow.io) to get your API key. You get **50 free credits** (= 200 verifications) with no credit card required. Each verification costs **0.25 credits**.

Paid plans start at **$17/mo** for 1,000 credits up to **$497/mo** for 100,000 credits. See [pricing](https://enrow.io/pricing).

## Documentation

- [Enrow API documentation](https://docs.enrow.io)
- [Full Enrow SDK](https://github.com/EnrowAPI/enrow-rust) -- includes email finder, phone finder, reverse email lookup, and more

## License

MIT -- see [LICENSE](LICENSE) for details.
