## Formatting

### Bold and Italic

| Markdown Example      | Valid/Invalid | Reason                                                                 |
|------------------------|---------------|------------------------------------------------------------------------|
| `*test_yeah*`          | Valid         | Surrounded correctly, no leading/trailing whitespace.                 |
| `* test_yeah*`         | Invalid       | Leading whitespace inside the formatting.                             |
| `*test yeah*`          | Valid         | Multiple words allowed, no leading/trailing whitespace.               |
| `*test_yeah *`         | Invalid       | Trailing whitespace inside the formatting.                            |
| `t_italic_t`           | Invalid       | Not surrounded properly by `*` or `_`.                                |
| `* word*`              | Invalid       | Leading whitespace breaks formatting.                                 |
| `test*test yeah*`      | Invalid       | No whitespace before the first `*`.                                   |
| `*test yeah*test`      | Invalid       | No whitespace after the last `*`.                                     |
| `test *yeah tests* yeah` | Valid       | Formatting surrounded by spaces works as expected.                    |
