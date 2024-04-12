# Barcode Generator for Device IDs

## Overview
This program is designed to generate a barcode from a Device ID retrieved from the network interfaces on a Windows system. It uses the Code128 barcode standard, which is suitable for encoding alphanumeric data. The barcode is then saved as a PNG file, and optionally opened with the default image viewer.

## Functionality
- **Retrieves Device ID:** The program uses the `netsh mbn show interface` command to find the last Device ID that consists only of numeric characters.
- **Barcode Generation:** Adds a required start character for Code128 barcodes and generates a barcode image from the Device ID.
- **Image Saving and Viewing:** Saves the barcode as a PNG file and attempts to open it with the system's default image viewer.

## Code Explanation
- **Command Execution:** The command `netsh mbn show interface | findstr /R "[0-9][0-9]*$"` is executed to filter the output to lines that end with numeric characters.
- **Barcode Processing:** Adds a 'Ɓ' (Code128 start character for character set B) to the beginning of the Device ID to comply with barcode standards.
- **Image Creation:** Generates a barcode image by setting pixels based on the barcode data, where '0' represents white and '1' represents black.
- **Error Handling:** Proper error handling is implemented for scenarios where the barcode cannot be generated or saved.

## Requirements
- **Rust Environment:** Ensure you have Rust installed on your machine.
- **External Crates:** Uses `barcoders` for barcode generation, `subprocess` for command execution, and `image` for image handling.

## How to Run
1. **Compile the Program:**
   ```bash
   cargo build --release
2. Or just run barcode_generator\target\release\barcode_generator.exe
