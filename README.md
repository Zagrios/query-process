# Query-Process
<a href="https://github.com/Zagrios/query-process/actions"><img alt="GitHub CI Status" src="https://github.com/Zagrios/query-process/workflows/CI/badge.svg"></a>

 `query-process` is a native Node.js npm library built with NAPI-RS, designed for querying information about external processes. Its initial capability includes checking if a process is running with elevated privileges, with plans for additional features in future updates.

## Installation

```
npm install query-process
```

## Configuration
`query-process` is configured to support multiple platforms, as specified in the `napi` section of `package.json`. This includes various architectures such as Linux, Windows, and Android.

## Usage

After installation, you can require query-process in your Node.js application.

```js
import queryProcess from 'query-process';
```
```js
const queryProcess = require('query-process');
```

# Features
The library currently supports:
- `isElevated`: Synchronously checks if a process is running with elevated privileges.

More features are planned for future releases.

## Examples

### isElevated
```js
import { isElevated } from 'query-process';

try {
  const elevated = isElevated();
  console.log(elevated);
} catch (err) {
  console.error(err);
}
```

# Test or Contributing

- Clone this repo
- Install latest stable Rust
- Install Node.js 10+
- Install dependencies with `npm install`
- Build Rust bindings with `npm run build`
- Run `npm test`

## Release package

We use GitHub actions to automatically publish npm packages.

```bash
# 1.0.0 => 1.0.1
npm version patch

# or 1.0.0 => 1.1.0
npm version minor
```

# License
`query-process` is made available under the MIT License. For more information, see the LICENSE file in the repository.


