$all_providers = 
    "address", "binaryfile", "choice", "code",
    "cryptographic", "date", "development",
    "file", "finance", "food", "hardware",
    "internet", "numeric", "path", "payment",
    "person", "science", "text", "transport"


foreach ($feature in $all_providers)
{
    Invoke-Expression -Command "cargo test --no-default-features -F 'all_locales $feature'"
}
