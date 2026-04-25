# Configuration

!!! note "Fill this in"
    Document each configuration option your project exposes. The structure below
    is a starting point — adapt it to your actual config surface.

## Overview

{{crate_name}} can be configured via:

1. A config file (`{{crate_name}}.toml` or `.{{crate_name}}.toml` in the project root)
1. Environment variables prefixed with `{{crate_name | upcase}}_`
1. Command-line flags (highest precedence)

## Options

### `option_name`

**Type:** `string`
**Default:** `"default_value"`
**Environment variable:** `{{crate_name | upcase}}_OPTION_NAME`

Description of what this option controls and when you would change it.

```toml
# {{crate_name}}.toml
option_name = "custom_value"
```

______________________________________________________________________

### `another_option`

**Type:** `bool`
**Default:** `false`
**Environment variable:** `{{crate_name | upcase}}_ANOTHER_OPTION`

Description of what this option controls.

```toml
another_option = true
```

## Example config file

```toml
# {{crate_name}}.toml
option_name = "custom_value"
another_option = true
```
