# Serial Port GUI

A graphical user interface application for serial port communication built with Rust and eframe.

![Alt text](https://example.com/path/to/image.png)

## Requirements

- Rust 1.70.0 or newer
- Windows OS (tested on Windows 10/11)

## Dependencies

- eframe = "0.31.1" - Egui framework for GUI
- serialport = "4.7.2" - Serial port communication
- winres = "0.1" - Windows resource handling

## Installation

1. Install Rust:
   - Download and run [rustup-init.exe](https://rustup.rs/)
   - Follow the installation prompts
   - Verify installation by opening a new terminal:
   ```bash
   rustc --version
   cargo --version
   ```

## Usage

### Getting Started
1. Launch the application (SerialProject.exe)
2. The main window will display available COM ports in a dropdown menu
3. Select your desired COM port from the list
4. Adjust the baud rate if needed (default is 9600)
5. Click "Connect" to establish a connection with the device

### Communication
- **Sending Data:**
  - Type your message in the text input field
  - Press Enter or click "Send" button to transmit
  - A newline character (\n) is automatically added to each message
  
- **Receiving Data:**
  - Incoming data appears automatically in the bottom text area
  - Data is displayed in real-time
  - Use the "Clear" button to reset the display

### Connection Management
- Monitor connection status through the status message
- Click "Disconnect" before closing the application
- Use the refresh button (🔄) to update the port list

## Controls

### Port Management
- **Port Selection Dropdown:** Lists all available COM ports
- **🔄 Button:** Updates the list of available ports
- **Connect/Disconnect Button:** Toggles port connection
- **Status Indicator:** Shows current connection state

### Data Settings
- **Baud Rate Control:** 
  - Adjustable from 300 to 115200
  - Common values: 9600, 19200, 38400, 57600, 115200
  - Can be changed before connection

### Data Handling
- **Send Text Field:** 
  - Enter data to transmit
  - Supports Enter key for quick sending
  
- **Received Data Area:**
  - Scrollable text display
  - Shows incoming data in real-time
  - "Clear" button to reset the display

## Features

### Connection Features
- Automatic port detection
- Real-time port status updates
- Configurable baud rate (300-115200)
- Connection status monitoring

### Data Features
- Real-time data reception display
- Text-based data transmission
- Enter key support for sending
- Clear received data option
- Automatic newline handling

### Interface Features
- Clean, intuitive GUI
- Port connection status indicator
- Auto-refresh port list capability
- Responsive data display

## Building

### Prerequisites
- Rust toolchain (latest stable version)
- Cargo package manager

### Build Commands

For development:
```bash
cargo build
