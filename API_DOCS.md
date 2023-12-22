# rytm external API docs

## How to read this documentation?

Words in these wrapped with `<..>` should be read as placeholders for the actual values.
For example `<selector>` should be read as `query`, `send`, `get` or `set`.

Words in these wrapped with `[]` should be read as optional values.
For example `[<parameter>]` should be read as `<parameter>` or nothing.

Words in these wrapped with `<..>` may include attached range of values to denote the valid range of an index or a parameter.
For example `<index 0..=127>` should be read as an integer between 0 and 127.
Ranges can be inclusive or exclusive. An exclusive range is denoted with `..` and an inclusive range is denoted with `..=`.

MaxMSP types will be denoted as `int`, `float`, `symbol` and `list`.

- `<selector>` A symbol which defines an operation. Available selectors are:
  - `query` Queries the device for data.
  - `send` Sends data to the device.
  - `get` Gets data from the `rytm` external.
  - `set` Sets data to the `rytm` external.
- `<object-type>` A symbol which defines the type of the object.
  - `pattern` A pattern.
  - `kit` A kit.
  - `sound` A pool sound.
  - `global` A global setting.
  - `settings` A settings structure.
  - `pattern_wb` The pattern from the work buffer.
  - `kit_wb` The kit from the work buffer.
  - `sound_wb` A sound from the work buffer.
  - `global_wb` The global setting from the work buffer.
- `<identifier>` A symbol which defines a parameter name. Usually followed by a `<parameter>` when setting data.
- `<enum>` A symbol which defines an enumeration. Enumerations are expressed in the format of `<enum-type>:` or `<enum-type>:<enum-value>`.
- `<parameter>` An integer or float parameter.
- `<element>` A sub element of a type.
- `<index>` An integer index.

## Query format

The query format is used to query data from the device.

`query <object-type> [<index>]`

Examples:

- `query pattern 1`
- `query settings`
- `query sound_wb 0`
- `query global_wb`

## Send format

The send format is used to send data to the device.

`send <object-type> [<index>]`

Examples:

- `send pattern 1`
- `send settings`
- `send sound_wb 0`
- `send global_wb`

## Get format

The get format is used to get data from the `rytm` external.

### Pattern

Accepted formats:

- `get pattern <index 0..=127> <identifier>`
- `get pattern <index 0..=127> <enum>`
- `get pattern <index 0..=127> <track-index 0..=12> <identifier>`
- `get pattern <index 0..=127> <track-index 0..=12> <enum>`
- `get pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> <identifier>`
- `get pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> <enum>`
- `get pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockget <identifier>`
- `get pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockget <enum>`

### Pattern work buffer

Accepted formats:

- `get pattern_wb <identifier>`
- `get pattern_wb <enum>`
- `get pattern_wb <track-index 0..=12> <identifier>`
- `get pattern_wb <track-index 0..=12> <enum>`
- `get pattern_wb <track-index 0..=12> <trig-index 0..=63> <identifier>`
- `get pattern_wb <track-index 0..=12> <trig-index 0..=63> <enum>`
- `get pattern_wb <track-index 0..=12> <trig-index 0..=63> plockget <identifier>`
- `get pattern_wb <track-index 0..=12> <trig-index 0..=63> plockget <enum>`

### Kit

Accepted formats:

- `get kit <index 0..=127> <identifier>`
- `get kit <index 0..=127> <enum>`
- `get kit <index 0..=127> <element> <element-index>`
- `get kit <index 0..=127> sound <sound-index 0..=11> <identifier> [<parameter>]`
- `get kit <index 0..=127> sound <sound-index 0..=11> <enum> [<parameter>]`

### Kit work buffer

Accepted formats:

- `get kit_wb <identifier>`
- `get kit_wb <enum>`
- `get kit_wb <element> <element-index>`
- `get kit_wb sound <sound-index 0..=11> <identifier> [<parameter>]`
- `get kit_wb sound <sound-index 0..=11> <enum> [<parameter>]`

### Sound

Accepted formats:

- `get sound <index 0..=11> <identifier> [<parameter>]`
- `get sound <index 0..=11> <enum> [<parameter>]`

### Sound work buffer

Accepted formats:

- `get sound_wb <index 0..=11> <identifier> [<parameter>]`
- `get sound_wb <index 0..=11> <enum> [<parameter>]`

### Global

Accepted formats:

- `get global <index 0..=3> <identifier> [<parameter>]`
- `get global <index 0..=3> <enum> [<parameter>]`

### Global work buffer

Accepted formats:

- `get global_wb <identifier> [<parameter>]`
- `get global_wb <enum>`

### Settings

Accepted formats:

- `get settings <identifier> [<parameter>]`
- `get settings <enum>`

## Set format

The set format is used to send data to the `rytm` external.

### Pattern

Accepted formats:

- `set pattern <index 0..=127> <identifier> <parameter>`
- `set pattern <index 0..=127> <enum>`
- `set pattern <index 0..=127> <track-index 0..=12> <identifier> <parameter>`
- `set pattern <index 0..=127> <track-index 0..=12> <enum>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> <identifier> <parameter>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> <enum>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockset <identifier> <parameter>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockset <enum>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockclear <identifier>`
- `set pattern <index 0..=127> <track-index 0..=12> <trig-index 0..=63> plockclear <enum>`

### Pattern work buffer

Accepted formats:

- `set pattern_wb <identifier> <parameter>`
- `set pattern_wb <enum>`
- `set pattern_wb <track-index 0..=12> <identifier> <parameter>`
- `set pattern_wb <track-index 0..=12> <enum>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> <identifier> <parameter>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> <enum>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> plockset <identifier> <parameter>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> plockset <enum>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> plockclear <identifier>`
- `set pattern_wb <track-index 0..=12> <trig-index 0..=63> plockclear <enum>`

### Kit

Accepted formats:

- `set kit <index 0..=127> <identifier> <parameter>`
- `set kit <index 0..=127> <enum>`
- `set kit <index 0..=127> <element> <element-index> <enum>`
- `set kit <index 0..=127> sound <sound-index 0..=11> <identifier> <parameter> [<parameter>]`
- `set kit <index 0..=127> sound <sound-index 0..=11> <enum> [<parameter>]`

### Kit work buffer

Accepted formats:

- `set kit_wb <identifier> <parameter>`
- `set kit_wb <enum>`
- `set kit_wb <element> <element-index> <enum>`
- `set kit_wb sound <sound-index 0..=11> <identifier> <parameter> [<parameter>]`
- `set kit_wb sound <sound-index 0..=11> <enum> [<parameter>]`

### Sound

Accepted formats:

- `set sound <index 0..=11> <identifier> <parameter> [<parameter>]`
- `set sound <index 0..=11> <enum> [<parameter>]`

### Sound work buffer

Accepted formats:

- `set sound_wb <index 0..=11> <identifier> <parameter> [<parameter>]`
- `set sound_wb <index 0..=11> <enum> [<parameter>]`

### Global

Accepted formats:

- `set global <index 0..=3> <identifier> <parameter>`
- `set global <index 0..=3> <enum> [<parameter>]`

### Global work buffer

Accepted formats:

- `set global_wb <identifier> <parameter>`
- `set global_wb <enum> [<parameter>]`

### Settings

Accepted formats:

- `set settings <identifier> <parameter>`
- `set settings <enum>`

## Identifiers

## Common identifiers

- `name` -> Name of the object. Type: `symbol` Range: `Only 15 ascii characters are allowed.`
- `index` -> Index of the object. Type: `int`.
- `version` -> Version of the object. Type: `int`.
- `iswb` -> Is work buffer. Type: `int` Range: `0..=1`

### Pattern

- `masterchg` -> Master length change. Type: `int` Range: `1..=1024`
- `masterlen` -> Master length. Type: `int` Range: `1..=1024`
- `kitnumber` -> Kit number. Type: `int` Range: `0..=127`
- `swingamount` -> Swing amount. Type: `int` Range: `50..=80`
- `globalquantize` -> Global quantize. Type: `int` Range: `0..=127`
- `patternbpm` -> Pattern BPM. Type: `float` Range: `30.0..=300.0`
