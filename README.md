# HarmonyLink: View

[![GitHub release](https://img.shields.io/github/release/Jordonbc/HarmonyLinkView)](https://github.com/Jordonbc/HarmonyLinkView/releases/)
![open issues](https://img.shields.io/github/issues-raw/Jordonbc/HarmonyLinkView)
![Open Source](https://badges.frapsoft.com/os/v1/open-source.svg?v=104)
![made-with-rust](https://img.shields.io/badge/Made%20With-Rust-Green)
![GitHub last commit](https://img.shields.io/github/last-commit/Jordonbc/HarmonyLinkView)

HarmonyLink: View is an innovative viewer application created using the Rust programming language. It provides real-time access to critical system metrics on handheld devices, enhancing your gaming experience and immersing you in a personalized gameplay environment. This viewer application is specifically tailored for the Steam Deck.

## Features

- Device Identification: HarmonyLink: View enables games to identify the specific handheld device they are operating on.
- Real-Time Metrics: Gain access to real-time data about the handheld device's power usage and docking status through HarmonyLink: View, creating a more responsive and immersive gaming experience.
- Cross-Platform Compatibility: HarmonyLink: View extends its functionality to Windows games running on Proton/Wine, preparing it for a wide array of future handheld devices.

## How It Works

HarmonyLink: View operates as a standalone viewer application. It connects to the HarmonyLink server running on the host side (native Linux or Windows) and provides a user-friendly interface to monitor the real-time metrics. The server, a Rust DLL, runs on the host system, and games access the metrics via an API, running natively or through Proton.

Developers and modders can easily implement GET and POST requests from the API to integrate system metrics into games. The power metrics and docking status can be used to adapt the game's quality settings, providing a more customized gaming experience.

## Building from Source

To build HarmonyLink: View from source, follow these steps:

1. Install Rust and Cargo on your development machine. Instructions can be found at [https://www.rust-lang.org/](https://www.rust-lang.org/).
2. Clone the repository: `git clone https://github.com/Jordonbc/HarmonyLinkView.git`
3. Navigate to the project directory: `cd HarmonyLinkView`
4. Install the necessary dependencies by running: `cargo install tauri-cli`
5. Build the application by running: `cargo tauri build`
6. After a successful build, you will find the executable for your platform in the `src-tauri/target/release` directory.
7. Run the HarmonyLink: View application on your handheld device.

## Contributions and Feedback

Your feedback, questions, and suggestions are highly appreciated. If you'd like to contribute to the HarmonyLink: View project, feel free to submit pull requests. Let's work together to enhance the handheld gaming experience!

## License

HarmonyLink: View is released under the [GNU General Public License v3.0 or later](https://www.gnu.org/licenses/gpl-3.0.en.html). Please refer to the license file for more information.
