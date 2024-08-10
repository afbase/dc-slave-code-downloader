# dc-slave-code-downloader

This Rust application crawls 116 pages from the Library of Congress website and downloads the largest JPEG image from each page.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Setup

1. Clone this repository:
```shell
git clone https://github.com/your-username/jpeg_crawler.git
cd jpeg_crawler
```

2. Build the project:
```shell
cargo build --release
```

## Usage

Run the application:

```shell
RUST_LOG=info cargo run --release
```

The application will start crawling the pages and downloading images. Progress and any errors will be logged to the console.

Downloaded images will be saved in the current directory with filenames like \`1.jpg\`, \`2.jpg\`, etc., corresponding to the page numbers.

## Configuration

You can adjust the logging level by changing the \`RUST_LOG\` environment variable. Available levels are: error, warn, info, debug, trace.

## Error Handling

The application includes error handling for network issues, parsing problems, and file I/O errors. Check the console output for any error messages.

## Performance

The crawler uses asynchronous operations for improved performance. It can handle multiple downloads concurrently.

## License

This project is open-source and available under the MIT License.
EOF

echo "Project setup complete. You can now build and run the JPEG crawler."