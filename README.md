# WASM JSON Sorter

This project demonstrates sorting JSON data by name in the browser using WebAssembly (WASM) and Rust.

## Table of Contents

- [Project Overview](#project-overview)
- [Requirements](#requirements)
- [Installation](#installation)
- [Running the HTTP Server](#running-the-http-server)
- [Building the WebAssembly](#building-the-webassembly)
- [Testing in the Browser](#testing-in-the-browser)
- [License](#license)

## Project Overview

The WASM JSON Sorter allows you to upload a JSON file and sort it by the name field using a Rust WebAssembly module. The Rust code compiles to WASM using wasm-pack, and the WebAssembly module is executed in the browser.

### Example JSON File Format:

You can use this site to get a file in below format: https://examplefile.com/public/.code/json
Just download a file and upload it locally.

```json
[
	{
		"name": "Maiya Kessler",
		"email": "osbaldo.mosciski@yahoo.com",
		"address": "6760 Mraz Islands Suite 733\nLukashaven, MS 31957",
		"phone": "1-782-558-6681 x4828",
		"website": "https://examplefile.com"
	},
	{
		"name": "Fiona Robel",
		"email": "jlynch@hotmail.com",
		"address": "54608 Sabrina Ranch\nBerneiceton, VT 46563-8385",
		"phone": "(939) 231-5930 x95008",
		"website": "https://examplefile.com"
	}
]
```

## Requirements

Before running the project, make sure you have the following installed:

- Rust
- wasm-pack for compiling Rust to WebAssembly
- Node.js (if you plan to use http-server to run the project)

## Installation

Clone the repository:

```bash
git clone https://github.com/yourusername/wasm-json-sorter.git
cd wasm-json-sorter
```

Install wasm-pack (if not already installed):

```bash
cargo install wasm-pack
```

## Building the WebAssembly

Build the WASM module using wasm-pack:

```bash
wasm-pack build --target web
```

This will generate a `pkg/` folder containing the WebAssembly binary and the JavaScript glue code needed to run it in the browser.

## Running the HTTP Server

Install http-server globally (if you haven't already):

```bash
npm install -g http-server
```

Serve the project locally using http-server:

```bash
http-server .
```

This will serve the project on `http://localhost:8080`.

Open the project in the browser:

Navigate to `http://localhost:8080/index.html` in your browser to upload a JSON file and sort it by name.

## Testing in the Browser

After building the WebAssembly module and running the HTTP server, visit `http://localhost:8080` in your browser.

- Upload a properly formatted JSON file (example format shown above).
- The JSON data will be sorted by the name field and displayed in a list.

## File Structure

- `index.html`: The main HTML page that allows you to upload a JSON file and interact with the WebAssembly.
- `src/lib.rs`: The Rust source file that defines the sorting logic.
- `pkg/`: The generated WebAssembly and JavaScript files created by wasm-pack.
- `Cargo.toml`: Rust's package configuration file.
