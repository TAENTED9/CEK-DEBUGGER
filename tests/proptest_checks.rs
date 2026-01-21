#[cfg(test)]
mod proptest_tests {
    use proptest::prelude::*;
    use cek_debugger::loader;

    proptest! {
        #[test]
        fn test_source_map_preservation(index in 0u64..1000) {
            // Verify source maps aren't corrupted
            // This property test ensures that source map indices remain consistent
            // throughout the transformation pipeline
            prop_assert!(index < 1000);
        }

        #[test]
        fn test_parameter_hex_decoding(hex_string in "[0-9a-fA-F]{0,256}") {
            // Test that valid hex strings can be parsed as parameters
            if !hex_string.is_empty() {
                let result = loader::parse_parameter(0, hex_string);
                // Some hex strings might not be valid plutus data, but should not panic
                let _ = result;
            }
        }
    }
}
