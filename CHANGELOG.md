# Changelog


## [0.1.1-pre](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/compare/v0.1.0-pre..v0.1.1-pre) - 2025-01-05




### 🐛 Bug Fixes

- *(ci)* Attempt to fix pipeline issues - ([1100a7c](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/1100a7c46937617fc6ecddf75ff6399c3cc9c18c))


## [0.1.0-pre] - 2025-01-05




### Docs

- *(parser)* Add example from tests to parser doc - ([143bc0c](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/143bc0c32cddf63a1ff99b27eee1652b1af707bf))
- *(lexer)* Add spec for md behaviour - ([3d66e77](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/3d66e778620f4b2d57fe93dc078ed397c21b49ec))

### Lexer

- *(newline)* Lex newlines - ([a03720b](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/a03720b54ea74e862434d1cf4cb7087dd666b074))

### Parser

- *(whitespace)* Parse all whitespace cases - ([29877e2](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/29877e26834f32289436da2271bc2795f26cbd4b))
- *(header)* Parse header - ([9e743a4](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/9e743a4cf760a060f792d480e646d47054ad6399))
- *(fmt)* Reformat - ([e7c8e12](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/e7c8e129f503d502e8e6b886c0f68ddb269bb2bf))
- *(plaintext)* Parse basic plaintext - ([f850756](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/f8507561f0c81a99d97fa67aab62a388c1f496a1))

### Tests

- *(error)* Verifty that errors are constructed correctly - ([451494f](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/451494f3e59acd873a678530169dfa771c2eea3d))
- *(args)* Add test for parsing tilde expansion - ([0772f22](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/0772f221914e5ca91832a759e4b54bdcbb3428c6))
- *(gfm)* Enable all test cases - ([37bccfd](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/37bccfde9d24578b9bc7ef344cda6c46b11e09cc))
- *(gfm)* Add all 677 examples from spec - ([c0a63e1](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/c0a63e146ec487d3fec60da104e17752c959de67))
- *(iter)* Add unit tests - ([26462ba](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/26462ba51e7babbdd171e6602f62a5692f9aaea7))

### ↩️ Reverts

- *(lexer)* Remvove comprehensive test - ([38fc59f](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/38fc59f6025942382ff1476eaf1b71381e92496e))

### ✨ Features

- *(config)* Print string for configuration docs - ([4042eac](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/4042eacae45fc864b8dc2b680d01aa55ced61a0b))
- *(parser)* Parse tables - ([a234471](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/a234471e655dbfa8997aae3f7d4e185cf272d2a4))
- *(lexer)* Lex pipes - ([e7f47a2](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/e7f47a2b73a264283e00da2ce85caab87d4bdc63))
- *(parser)* Parse link details - ([641ca8e](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/641ca8ebeec675c2791586264d11b16dc00d0db7))
- *(lexer)* Lex opening carrot - ([24ecb13](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/24ecb13a88dbd6e7fa8655699c4c5c01ce33b4e8))
- *(parser)* Parse images - ([5ab081c](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/5ab081ca89ab6ee7cab40cf12bbeca58450790aa))
- *(lexer)* Lex ! - ([cffebab](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/cffebab82464d590fff232682eddee296d2a0c06))
- *(parser)* Parse Footnotes - ([88319bf](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/88319bf1db91502ede2bf981a4fe59eb9a848b1c))
- *(parser)* Parse links - ([6b4dedb](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/6b4dedbece656dacef0e73412da92decbd71a732))
- *(lexer)* Lex [ - ([242aef2](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/242aef2973ac31921ddfe84557a16575f69a2e28))
- *(parser)* Parse codeblocks - ([ba2dc6a](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/ba2dc6a993795a9ae3eef18bf7a6189073c4b78b))
- *(lexer)* Lex codeblocks - ([2c0bafd](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/2c0bafdd064e92fcfd2e32acd3b4ebf1753a53a6))
- *(parser)* Parse strikethrough - ([6dcc02f](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/6dcc02f401c08e600c7842c1c9671a78b61cd11a))
- *(lexer)* Lex tilde - ([d76624a](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/d76624a7e3e0022323e407e2729270bf0dcccfe7))
- *(parser)* Lex list entries - ([2b7ae51](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/2b7ae51065f0df59e917a010a6c6e5737834d7f0))
- *(lexer)* Lex plus minus - ([4761198](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/476119888e1c49b7d1dc970a2a8ca4ef6c5eebc1))
- *(parser)* Parse block quotes - ([c69cc99](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/c69cc99b4d78e0d5280f1394f7b8ac20d33c5f01))
- *(lexer)* Lex block quotes - ([9050874](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/90508741d1c67f528ef186ed117d54c505663944))
- *(args)* Print version character in front of version string - ([00a8c6d](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/00a8c6d67c86453c4b6ab39f4527a44a14b196e6))
- *(parser)* Parse asterisk and underscore - ([7494ed7](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/7494ed784f603bdb9f27fd05c195fd560689c339))
- *(lexer)* Lex asterisk and underscore - ([a769e32](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/a769e32fad96bf4e34135d131949f40b2ae0bb1e))
- *(lexer)* Lex escaped characters - ([bc0d16d](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/bc0d16d3faa5c099b71486f05387b246d8f0708a))
- *(parser)* Parse ordered lists - ([ab2891e](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/ab2891e892570b8b7722e926900e40ff18accb7d))
- *(lexer)* Lex numbers - ([a2563cc](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/a2563ccbfa21dd418f0368fdadb3be264ef238e0))
- *(convert)* Wrapper for conversion logic - ([36e8bdb](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/36e8bdbe2d68b20016842ddc09d64e0f89cd7710))
- *(headles_chrome)* Add basic wrapper - ([b0e5b96](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/b0e5b96d2a61d660cdebb2036ec188ae5aa8d071))
- *(config)* Add command to generate default config - ([bb9a307](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/bb9a307c1d7d8ce42c7223b1a540720a4a95cdd0))
- *(convert)* Early exit when converting to html - ([298e250](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/298e250ef5fd6edd05cb08d3f45f8b256fc013e2))
- *(args)* Canonicalize path when reading from stdin - ([4024d8f](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/4024d8f24ab4b601568bdf9404d39711ee85485d))
- *(lexer)* Improve error msg handling for ParseError - ([929ac7c](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/929ac7c5a60ad39eb651b1c94197b4ad4125b440))
- *(fs)* Add wrapper fn for writing to a file - ([040cdae](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/040cdaea0e00fdd78a0538e4bf4e52d9fc83b10c))
- *(logger)* Add missing logs - ([20702df](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/20702dfd24d4f88dba6b1a8a15fad0aaef2eeab2))
- *(logger)* Implement basic logging - ([98cf61c](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/98cf61c0dcaa2fbe9440cb010971008467228f7f))
- *(structure)* Add basic structure - ([24f5962](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/24f5962f1c5e9d7f49917410f7734fa8ba1e392c))
- *(fs)* Wrapper for basic file io - ([adf00e5](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/adf00e5e065adb409e1f28efabb7dfbf044c9a56))
- *(args)* Add basic structure for command parsing - ([719449b](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/719449bcb23c1f32a0c69e07bb75e17a10b3fda4))
- *(lexer)* Lex whitespaces and tabs - ([f9857b2](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/f9857b28267351462e56375c8c181cfc3edb4b1f))
- *(token)* Add all possible token types - ([cf0ffc9](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/cf0ffc91ced882040e519cc04e8fffc51091c137))
- *(lexer)* Recursive lexing for headers - ([375646c](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/375646caca0609dd5a52b3ef8c6aff2874d1739a))
- *(parser)* Impl basic parser - ([90fe289](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/90fe289d9019263f1184c317a8cdbf90e50a4aa1))
- *(lexer)* Lex atx headings - ([3f3778b](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/3f3778b37074c60277ded68885ffabcc9a7446e8))
- *(lexer)* Add basic iter - ([39dba80](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/39dba80360848c4e930a923ef13b7266308597c7))
- *(lexer)* Replace unwrap() calls with logic - ([0ea3445](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/0ea3445f027d91074798cdacf0e96b48dec8217c))
- *(lexer)* Lex escaped characters - ([eb6d623](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/eb6d623d6d2a70ce0696c2feb92f5125d3cbcdc4))
- *(lexer)* Lex text format - ([9ec7b74](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/9ec7b744876e34a7ffc84df6e0b9f345b8675de0))
- *(lexer)* Add comprehensive test covering entire lexer - ([9eceede](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/9eceede6de1c38dd4d8e11b67341d4f152074bf8))
- *(lexer)* Newlines behave like semicolons - ([123f65f](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/123f65fd2f18b1a985411a2d5990f9c191c25b0a))
- *(lexer)* Lex headers - ([5d359fa](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/5d359faea55abe5838e25a598f9148d7a81973ab))
- *(lexer)* Lex simple strings - ([c217135](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/c217135c25ee08172cb2067dc07770bd828e2e43))
- *(lexer)* Add basic lexer setup - ([d919500](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/d9195000b17f05a87849f11f7e591bfb1d95c205))
- *(lexer)* Lex paragraph - ([8db0777](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/8db077747633bd20b8224a0663ec22e7a9e8566d))
- *(lexer)* Lex header - ([686abff](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/686abff5386f3ed1c896eb7bc4c1dce18b75dca5))

### 🎉 Initial Commit

- *(rust)* Init with rust - ([4f2ac97](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/4f2ac9702169993dfb7150c6ee3a3bd876e3f9f7))
- *(template)* Git-cliff template - ([946c71a](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/946c71ae6a09016c0c331a97c570baedc95d2a29))

### 🐛 Bug Fixes

- *(ci)* Remove duplicate keey - ([720bf56](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/720bf56d82f5c5e2d7221ec7bcfcf1f1f8096beb))
- *(run)* Convert parsed result instead of raw file content - ([eb542e2](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/eb542e207d07d9e4e51677adfc9a36f3f88b6bfa))
- *(parser)* Remove duplicate code - ([c52a7d0](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/c52a7d00952aca45594c020349e137a33b412df0))
- *(tests)* Replace → with \t - ([62600a6](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/62600a6f6a202189366cfa4331f25fe0a1bb7fe0))
- *(lexer)* Strip paragraph texts after rendering contents - ([bc13a68](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/bc13a6802416a1f2b99a7c82e32ab51782374838))
- *(lexer)* Incorrect logging - ([9d35eba](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/9d35ebab417c905b02855e703dafc3582f918545))
- *(ci)* Always return exit-code 0 to continue after tests - ([515c8ae](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/515c8aec60cd4a81843a3734b18990a164ca9166))
- *(lexer)* Remove public modifier from fn - ([c69497f](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/c69497f62c2be012881c5ef147d39a842c770b34))
- *(main)* Fix sample - ([5e59dc7](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/5e59dc7eee62e939a3d4da68c8d0cac07b15ab7c))

### 🔧Chores

- *(release)* Prepare for v0.1.0-pre - ([aaa47e5](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/aaa47e5fb3f1255acfeb92210a15380a66f3d63e))
- *(release)* Prepare for v0.1.0-pre - ([a67dc0a](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/a67dc0abe84a4b3129878be693fb81c357bcb258))
- *(release)* Remove unwanted LICENSE - ([1f58785](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/1f587850a5c5cc838c5959183e05fc97712172d1))
- *(release)* Auto replace versions in config files - ([7518fe5](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/7518fe52ee348be306821a6d5bc19c5926f591e3))
- *(release)* Add npm stuff idk pray - ([cae5aef](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/cae5aef7baa70ca01a623a644e62e6cd7a4575cf))
- *(release)* Add pypi stuff - ([a6e55cc](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/a6e55ccbdf543f84ab8ecde2e7eb3691af38c8ef))
- *(repo)* Add editorconfig - ([fc05e42](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/fc05e422da2cb4793e6e494d443470d5af5234aa))
- *(repo)* Add rust toolchain - ([cb53b68](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/cb53b681f1b2b35707d555ee4719500d81d2db94))
- *(release)* Workflow to release to all kinds of pm's - ([60cb20f](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/60cb20ff1b8602f1892d67a69ed2250f3901c1ab))
- *(release)* Prepare cargo.toml for release - ([8f09088](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/8f09088c8c1a9be3e45a3180ce76635b4eb47484))
- *(completions)* Add small programm to generate shell completions - ([70082a8](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/70082a8f85fbb877f7af12b737deaf19e595ca3d))
- *(mangen)* Add small programm to generate mangpage for smd - ([7d90bec](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/7d90becd5dc1c589229b5acd8b5807332e41a38f))
- *(fmt)* Reformat - ([dd5357e](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/dd5357e04324c52de1531c8d48812b7cdcb00bcd))
- *(clippy)* Apply linting suggestions - ([0a459f3](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/0a459f331a1c83cbbfbf42ff9d9edb46b3724338))
- *(ci)* Cache workflow builds - ([8cc942a](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/8cc942a54105c777e17c75ae7066954a549662d0))
- *(tests)* Fix typo - ([3b2aa5c](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/3b2aa5cd30191f52611bf9cad3a0829325e71b70))
- *(repo)* Update link to codecov page - ([3d26b82](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/3d26b82c9ce74aa225a4e499fcff71980162e03f))
- *(ci)* Push code coverage results even if tests are failing - ([78265ce](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/78265cec1583a2031706aee5e412c8622ccbe986))
- *(nix)* Add cargo-tarpaulin dep - ([9ab659b](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/9ab659b4d6798c37a606302c7f6b4de65e8b95f8))
- *(ignore)* Add html and pdf to .gitignore - ([cfc4453](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/cfc4453980f389b33d5ac4a912c185c463fd4863))
- *(ci)* Set rustup channel to nightly - ([f9a6490](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/f9a64906f640c6defdb3ae4428904340c1dec6f8))
- *(flake)* Add act and remove unwanted deps - ([1dcad3a](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/1dcad3a07a5322978dfbfab29aff2cc2483c2094))
- *(repo)* Move and update ROADMAP - ([82d9ff4](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/82d9ff4d9ce22799f1a6a7bed09272ca9fbace54))
- *(repo)* Remove color-line from README - ([1e2cd57](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/1e2cd570f270d6d1c71bbe77fb5779a9f5dbbac8))
- *(reformat)* Change rusfmt settings - ([9d6b5c7](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/9d6b5c7f64b4a9fd20c4cd5ccfddb76d376bc375))
- *(repo)* Reformat with widht=80 - ([c71219a](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/c71219a13d7f04396ec1129f9e177d97fdc47bee))
- *(ci)* Only run tests for binary in main workflow - ([5ac6833](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/5ac6833871be723eef871acf4d488b7ad7d9a2aa))
- *(repo)* Update icons in README - ([be3bc4e](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/be3bc4ea120ce21bf9d69e54db6aff2a4d382774))
- *(repo)* Fix typo in link - ([e18ce73](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/e18ce73e80a3e7a21f2e039dc7a6224658a44858))
- *(repo)* Update colors and upload results to code-cov - ([0849665](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/0849665020e4f6286a477393c6514b8e69b69be7))
- *(repo)* Fix styling and workflow - ([e78f4f4](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/e78f4f4fde968d2e5216971743cc7b5e9870566d))
- *(repo)* Update README and add test/code-cov workflow - ([334ff74](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/334ff7421bdaf9bd09b36a32b1437178415239e4))
- *(repo)* Add roadmap to README - ([7a1a2f3](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/7a1a2f3112fad7509486c9d7986ff4cdaa4f7b2f))

### 🚜 Refactoring

- *(lexer)* Remove scuffed unwrap logic - ([11f8cbf](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/11f8cbf5d2d332a70d58c9d797f1226712469131))
- *(gfm)* Move Parser to `lib` - ([751d9f4](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/751d9f417a203863af534b3becf4b4d7c0438433))
- *(gfm)* Extract transpiler into own workspace - ([fba20e0](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/fba20e073faaafdd27743a17888a60916f0bc43a))
- *(config)* Move initialization to core - ([8977a41](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/8977a41c0a97ab120891805ec2569e08ca7a62e5))
- *(parser)* Reexport crate insteaf of forwarding - ([d719813](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/d719813037ebb072c43a9c56bb6b3b5c4a7f41b0))
- *(repo)* Completly rework structure and reset lexer - ([5a717b4](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/5a717b4e6b5bee955de66b923594c746c99f5442))
- *(lexer)* Make formatting logic more compact - ([994d5d3](https://github.com/Flokkq/https://github.com/orhun/git-cliff/blob/main/cliff.toml/commit/994d5d3f2ef2c15dde0f71bd5355e2d03b8322a0))
<!-- generated by git-cliff -->
