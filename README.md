# Sunshine Resolution
Small semi-automatic cli util to change resolution while streaming via sunshine.
Primarily made to learn some rust :-)

## Usage
1. Download and unzip archive anywhere you want.
2. Check config.toml file to change resolutions and/or path to sunshine.conf.
3. Run sunshine-resolution.exe with admin privileges. It will inject callback to this app (needs to be done only once).  
   3.1. Re-run .exe file if location of the sunshine or this util file has changed.

P.S. Even though it is made for automatic usage you can still use as a cli program.
```
$ .\sunshine-resolution.exe --help
Usage: sunshine-resolution.exe [OPTIONS]

Options:
  -d, --default  Change resolution to default value
  -c, --custom   Change resolution to custom value
  -h, --help     Print help
  -V, --version  Print version
```
