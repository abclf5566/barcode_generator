# Barcode Generator for Device IMEI on Windows 10

## Overview
This program generates a barcode from a Device IMEI, retrieved from the network interfaces on a Windows 10 system. It utilizes the Code128 barcode standard, ideal for encoding alphanumeric data. The generated barcode is saved as a PNG file and can be automatically opened using the default image viewer.

## Functionality
- **Retrieves Device IMEI:** Executes the `netsh mbn show interface` command to locate the last numeric Device ID from network interface details.
- **Barcode Generation:** Precedes the numeric Device IMEI with a 'Ɓ', the required start character for Code128 barcodes, ensuring compliance with barcode specifications.
- **Image Saving and Viewing:** The barcode is saved as a PNG file. The program also attempts to automatically open this file with the system's default image viewer.

## Code Explanation
- **Command Execution:** Runs `netsh mbn show interface | findstr /R "[0-9][0-9]*$"` to extract lines that end with numeric characters, identifying the Device IMEI.
- **Barcode Processing:** Incorporates the 'Ɓ' character at the start of the Device IMEI to meet Code128 standards.
- **Image Creation:** Constructs a barcode image by setting pixel values based on barcode encoding, where '0' denotes white and '1' denotes black pixels.
- **Error Handling:** Implements robust error handling to manage scenarios where barcode generation or saving fails.

## Requirements
- **Rust Environment:** Rust must be installed on your computer to compile and run this program.
- **External Crates:** Utilizes `barcoders` for barcode generation, `subprocess` for command execution, and `image` for image manipulation.

## How to Run
1. **Compile the Program:**
   Compile the code to create an executable. Open a terminal and navigate to the project directory, then run:
   ```bash
   cargo build --release
