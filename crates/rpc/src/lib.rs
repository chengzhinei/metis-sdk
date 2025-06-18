pub mod client;
pub mod tx;
pub mod message;
pub mod web_socket;
pub mod query;
pub mod response;

/// A [`base64::Engine`] using the [`alphabet::STANDARD`] base64 alphabet
/// padding bytes when writing but requireing no padding when reading.
const B64_ENGINE: base64::engine::GeneralPurpose = GeneralPurpose::new(
    &alphabet::STANDARD,
    GeneralPurposeConfig::new()
        .with_encode_padding(true)
        .with_decode_padding_mode(DecodePaddingMode::Indifferent),
);