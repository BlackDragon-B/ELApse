# ELApse

ELApse is a tool used for processing and displaying the .ELA format from a dead rhythm game 

## Downloading

You can either download the latest release from the [releases](https://github.com/BlackDragon-B/ELApse/releases) or build it yourself using cargo
## Usage

```bash
ELApse --input <INPUT> --output <OUTPUT>

Options:
  -i, --input <INPUT>    Input to use
  -o, --output <OUTPUT>  Output to use
  -h, --help             Print help
  -V, --version          Print version
```
## Features
### Input
- [WARLS](https://github.com/Aircoookie/WLED/wiki/UDP-Realtime-Control) (--input udp://0.0.0.0:21324)
- Images (--input example.png)
- .uasset/.uexp (--input ELA_BP_Title_in_00.uasset)
- File containing raw rgb8 (--input file)
- Raw rgb8 from stdin (--input stdin)
### Output
- [WARLS](https://github.com/Aircoookie/WLED/wiki/UDP-Realtime-Control) (--output udp://0.0.0.0:21324)
- Built-in renderer built with [Piston](https://www.piston.rs/) (--output piston)
## Contributing

Feel free to open an issue if you think this tool is missing something critical. Pull requests are always welcome.

## License

ELApse is licensed under the  [GPLv3](https://choosealicense.com/licenses/gpl-3.0/) license.
