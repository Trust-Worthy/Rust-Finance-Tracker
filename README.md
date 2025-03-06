# Simple Ledger

A lightweight command-line ledger application built in Rust to track income and expenses efficiently. This project allows users to record transactions, view summaries, and manage financial records with ease.

## Features

- **Add Transactions**: Users can log income and expenses with date, category, and amount.
- **View Summary**: Display a complete history of transactions.
- **Filter by Date**: View transactions within a specified date range.
- **Total Income & Expenses**: Calculate the total income and expenses over time.
- **User-Friendly CLI**: Simple command-line interface for ease of use.

## Installation

To install and run the Simple Ledger, ensure you have Rust installed. Clone the repository and build the project:

```sh
git clone <repository_url>
cd simple-ledger
cargo build --release
```

## Usage

Run the application with:

```sh
cargo run
```

Follow the on-screen prompts to add transactions and view reports.

## Development Progress

- Implemented core transaction management functionality.
- Added error handling for parsing user input.
- Integrated date filtering for transactions.
- Fixed semantic issues in transaction summaries.
- Improved CLI experience with structured menus and options.

## Future Improvements

- CSV Import/Export support.
- Persistent storage using a database or file system.
- Enhanced reporting with graphical summaries.

## License

This project is licensed under the MIT License.
