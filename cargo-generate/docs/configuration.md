# Configuration

!!! note "Fill this in"
    Document each configuration option your project exposes. The structure below
    is a starting point — adapt it to your actual config surface.

## Overview

{{project_name}} can be configured via:

1. A config file (`{{project_name}}.toml` or `.{{project_name}}.toml` in the project root)
2. Environment variables prefixed with `{{project_name | upper}}_`
3. Command-line flags (highest precedence)

## Options

### `option_name`

**Type:** `string`  
**Default:** `"default_value"`  
**Environment variable:** `{{project_name | upper}}_OPTION_NAME`

Description of what this option controls and when you would change it.

```toml
# {{project_name}}.toml
option_name = "custom_value"
```

---

### `another_option`

**Type:** `bool`  
**Default:** `false`  
**Environment variable:** `{{project_name | upper}}_ANOTHER_OPTION`

Description of what this option controls.

```toml
another_option = true
```

## Example config file

```toml
# {{project_name}}.toml
option_name = "custom_value"
another_option = true
```
