---
type: Rust Module
title: secrets
resource: src/config/secrets.rs#L1-L393
generated:
  by: okf-rs/0.3.0
relationships:
  imports:
  - external/anyhow-context-result
  - external/keyring-entry
  - external/serde-deserialize-serialize
  - external/std-fmt
  - external/zeroize-zeroize-zeroizeondrop
  - external/super
  member_of:
  - packages/crustly
---

# Contains

- [SecretString](../../../classes/src/config/secrets/SecretString.md)
- [new](../../../functions/src/config/secrets/SecretString/new.md)
- [from_str](../../../functions/src/config/secrets/SecretString/from_str.md)
- [from_env](../../../functions/src/config/secrets/SecretString/from_env.md)
- [from_env_optional](../../../functions/src/config/secrets/SecretString/from_env_optional.md)
- [from_keyring](../../../functions/src/config/secrets/SecretString/from_keyring.md)
- [from_keyring_optional](../../../functions/src/config/secrets/SecretString/from_keyring_optional.md)
- [save_to_keyring](../../../functions/src/config/secrets/SecretString/save_to_keyring.md)
- [delete_from_keyring](../../../functions/src/config/secrets/SecretString/delete_from_keyring.md)
- [load_with_fallback](../../../functions/src/config/secrets/SecretString/load_with_fallback.md)
- [expose_secret](../../../functions/src/config/secrets/SecretString/expose_secret.md)
- [is_empty](../../../functions/src/config/secrets/SecretString/is_empty.md)
- [len](../../../functions/src/config/secrets/SecretString/len.md)
- [fmt](../../../functions/src/config/secrets/SecretString/fmt-debug/fmt.md)
- [fmt](../../../functions/src/config/secrets/SecretString/fmt-display/fmt.md)
- [serialize](../../../functions/src/config/secrets/SecretString/serialize/serialize.md)
- [deserialize](../../../functions/src/config/secrets/SecretString/deserialize-de/deserialize.md)
- [from](../../../functions/src/config/secrets/SecretString/from-string/from.md)
- [from](../../../functions/src/config/secrets/SecretString/from-str/from.md)
- [ProviderSecrets](../../../classes/src/config/secrets/ProviderSecrets.md)
- [new](../../../functions/src/config/secrets/ProviderSecrets/new.md)
- [from_env](../../../functions/src/config/secrets/ProviderSecrets/from_env.md)
- [load_with_fallback](../../../functions/src/config/secrets/ProviderSecrets/load_with_fallback.md)
- [save_to_keyring](../../../functions/src/config/secrets/ProviderSecrets/save_to_keyring.md)
- [delete_from_keyring](../../../functions/src/config/secrets/ProviderSecrets/delete_from_keyring.md)
- [has_any](../../../functions/src/config/secrets/ProviderSecrets/has_any.md)
- [count](../../../functions/src/config/secrets/ProviderSecrets/count.md)
- [default](../../../functions/src/config/secrets/ProviderSecrets/default/default.md)
- [test_secret_string_creation](../../../functions/src/config/secrets/test_secret_string_creation.md)
- [test_secret_string_debug](../../../functions/src/config/secrets/test_secret_string_debug.md)
- [test_secret_string_display](../../../functions/src/config/secrets/test_secret_string_display.md)
- [test_provider_secrets_empty](../../../functions/src/config/secrets/test_provider_secrets_empty.md)
- [test_provider_secrets_with_keys](../../../functions/src/config/secrets/test_provider_secrets_with_keys.md)
- [test_secret_string_from_env](../../../functions/src/config/secrets/test_secret_string_from_env.md)
- [test_secret_string_from_env_optional](../../../functions/src/config/secrets/test_secret_string_from_env_optional.md)
- [test_secret_string_serialize](../../../functions/src/config/secrets/test_secret_string_serialize.md)

# Imports

- `anyhow::{Context, Result}`
- `keyring::Entry`
- `serde::{Deserialize, Serialize}`
- `std::fmt`
- `zeroize::{Zeroize, ZeroizeOnDrop}`
- `super::*`

# Member of

- [crustly](../../../packages/crustly.md)