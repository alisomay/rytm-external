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

- `name` -> **symbol**, _Only 15 ascii characters are allowed_ (name of the object)
- `index` -> **int** (index of the object)
- `parentindex` -> **int** (index of the parent object if there is one)
- `version` -> **int** (version of the object)
- `iswb` -> **int**, _0..=1_ (is work buffer object)

### Pattern

- `masterchg` -> **int**, _1..=1024_
- `masterlen` -> **int**, _1..=1024_
- `kitnumber` -> **int**, _0..=127_
- `swingamount` -> **int**, _50..=80_
- `globalquantize` -> **int**, _0..=127_
- `patternbpm` -> **float**, _30.0..=300.0_

### Track

- `deftrignote` -> **int**, _0..=127_
- `deftrigvel` -> **int**, _0..=127_
- `deftrigprob` -> **int**, _0..=100_
- `steps` -> **int**, _1..=64_
- `quantizeamount` -> **int**, _0..=127_
- `sendsmidi` -> **int**, _0..=1_
- `euc` -> **int**, _0..=1_
- `pl2` -> **int**, _0..=63_
- `ro1` -> **int**, _0..=63_
- `ro2` -> **int**, _0..=63_
- `tro` -> **int**, _0..=63_

## Trig

- `enable` -> **int**, _0..=1_
- `retrig` -> **int**, _0..=1_
- `mute` -> **int**, _0..=1_
- `accent` -> **int**, _0..=1_
- `swing` -> **int**, _0..=1_
- `slide` -> **int**, _0..=1_
- `note` -> **int**, _0..=127_
- `vel` -> **int**, _1..=127_
- `retrigveloffset` -> **int**, _-128..=127_
- `soundlock` -> **int**, _0..=127_

## Kit

- `ctrlinmod1amt` -> **int**, _-128..=127_
- `ctrlinmod2amt` -> **int**, _-128..=127_
- `fxdeltime` -> **int**, _0..=127_
- `fxdelpingpong` -> **int**, _0..=1_
- `fxdelstereowidth` -> **int**, _-64..=63_
- `fxdelfeedback` -> **int**, _0..=198_
- `fxdelhpf` -> **int**, _0..=127_
- `fxdellpf` -> **int**, _0..=127_
- `fxdelrevsend` -> **int**, _0..=127_
- `fxdellev` -> **int**, _0..=127_
- `fxrevpredel` -> **int**, _0..=127_
- `fxrevdecay` -> **int**, _0..=127_
- `fxrevfreq` -> **int**, _0..=127_
- `fxrevgain` -> **int**, _0..=127_
- `fxrevhpf` -> **int**, _0..=127_
- `fxrevlpf` -> **int**, _0..=127_
- `fxrevlev` -> **int**, _0..=127_
- `fxcompthr` -> **int**, _0..=127_
- `fxcompgain` -> **int**, _0..=127_
- `fxcompmix` -> **int**, _0..=127_
- `fxcomplev` -> **int**, _0..=127_
- `fxlfospeed` -> **int**, _-64..=63_
- `fxlfofade` -> **int**, _-64..=63_
- `fxlfostartphase` -> **int**, _0..=127_
- `fxlfodepth` -> **float**, _-128.0..=127.99_
- `fxdistdov` -> **int**, _0..=127_
- `fxdistamt` -> **int**, _0..=127_
- `fxdistsym` -> **int**, _-64..=63_
- `fxdistdelpost` -> **int**, _0..=1_
- `fxdistrevpost` -> **int**, _0..=1_

## Sound

- `ispool` -> **int**, _0..=1_
- `iskit` -> **int**, _0..=1_
- `kitnumber` -> **int**, _0..=127_
- `accentlev` -> **int**, _0..=127_
- `ampattack` -> **int**, _0..=127_
- `amphold` -> **int**, _0..=127_
- `ampdecay` -> **int**, _0..=127_
- `ampoverdrive` -> **int**, _0..=127_
- `ampdelsend` -> **int**, _0..=127_
- `amprevsend` -> **int**, _0..=127_
- `amppan` -> **int**, _-64..=63_
- `amplev` -> **int**, _0..=127_
- `filtattack` -> **int**, _0..=127_
- `filthold` -> **int**, _0..=127_
- `filtdecay` -> **int**, _0..=127_
- `filtrelease` -> **int**, _0..=127_
- `filtcutoff` -> **int**, _0..=127_
- `filtres` -> **int**, _0..=127_
- `filtenvamt` -> **int**, _-64..=63_
- `lfospeed` -> **int**, _-64..=63_
- `lfofade` -> **int**, _-64..=63_
- `lfostartphase` -> **int**, _0..=127_
- `lfodepth` -> **float**, _-128.0..=127.99_
- `samptune` -> **int**, _-24..=24_
- `sampfinetune` -> **int**, _-64..=63_
- `sampnumber` -> **int**, _0..=127_
- `sampbitreduction` -> **int**, _0..=127_
- `sampstart` -> **float**, _0.0..=120.0_
- `sampend` -> **float**, _0.0..=120.0_
- `samploopflag` -> **int**, _0..=1_
- `samplev` -> **int**, _0..=127_
- `velmodamt` -> **int**, _-127..=128_
- `atmodamt` -> **int**, _-127..=128_
- `envresetfilter` -> **int**, _0..=1_
- `veltovol` -> **int**, _0..=1_
- `legacyfxsend` -> **int**, _0..=1_

## Global

- `kitreloadonchg` -> **int**, _0..=1_
- `quantizeliverec` -> **int**, _0..=1_
- `autotrackswitch` -> **int**, _0..=1_
- `routetomain` -> **int**, _0..=11_
- `sendtofx` -> **int**, _0..=11_
- `clockreceive` -> **int**, _0..=1_
- `clocksend` -> **int**, _0..=1_
- `transportreceive` -> **int**, _0..=1_
- `transportsend` -> **int**, _0..=1_
- `pgmchangereceive` -> **int**, _0..=1_
- `pgmchangesend` -> **int**, _0..=1_
- `receivenotes` -> **int**, _0..=1_
- `receiveccnrpn` -> **int**, _0..=1_
- `turbospeed` -> **int**, _0..=1_
- `metronomeactive` -> **int**, _0..=1_
- `metronomeprerollbars` -> **int**, _0..=16_
- `metronomelev` -> **int**, _0..=127_

## Settings

- `projectbpm` -> **float**, _30.0..=300.0_
- `selectedtrack` -> **int**, _0..=11_
- `selectedpage` -> **int**, _0..=3_
- `mute` -> **int**, _0..=11_
- `unmute` -> **int**, _0..=11_
- `fixedvelocity` -> **int**, _0..=1_
- `fixedvelocityamt` -> **int**, _0..=127_
- `samplerecorderthr` -> **int**, _0..=127_
- `samplerecordermon` -> **int**, _0..=1_

## Enums

<!-- pub mod pattern_enum_type {
    pub const SPEED: &str = "speed";
    pub const TIME_MODE: &str = "timemode";
} -->

### Pattern

- `speed`
  - **1x**
  - **2x**
  - **3/2x**
  - **3/4x**
  - **1/2x**
  - **1/4x**
  - **1/8x**
