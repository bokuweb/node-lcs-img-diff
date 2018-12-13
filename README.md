# node-lcs-img-diff
Image diff tool with LCS algorithm. port of [murooka/go-diff-image](https://github.com/murooka/go-diff-image)

[![CircleCI](https://circleci.com/gh/bokuweb/node-lcs-img-diff.svg?style=svg)](https://circleci.com/gh/bokuweb/node-lcs-img-diff)

## Requirements
- Node.js v8+

## Installation

``` bash
npm install lcs-img-diff
```

## Usage

```
lcs-img-diff path/to/before.png path/to/after.png --dist dist --prefix marked
```

## Example

| before.png        | after.png          |
| --------------- |---------------| 
| ![](https://github.com/bokuweb/node-lcs-img-diff/blob/master/test/images/before.png?raw=true) | ![](https://github.com/bokuweb/node-lcs-img-diff/blob/master/test/images/after.png?raw=true) |

`lcs-img-diff` outputs marked before and after images too. 

| marked_before.png        | marked_after.png          |
| --------------- |---------------|
| ![](https://github.com/bokuweb/node-lcs-img-diff/blob/master/test/expected/marked_before.png?raw=true) | ![](https://github.com/bokuweb/node-lcs-img-diff/blob/master/test/expected/marked_after.png?raw=true) |

## LICENSE

The MIT License (MIT)

Copyright (c) 2018 @bokuweb

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
