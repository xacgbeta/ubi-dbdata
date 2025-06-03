# Ubisoft Denuvo dbdata.dll

This project provides a Rust implementation of a DLL (`dbdata.dll`) that emulates the Ubisoft Denuvo token interface. It is intended for research, reverse engineering, and interoperability purposes.

## Usage

1. **Build the DLL:**

   ```sh
   cargo build --release
   ```

   The resulting DLL will be located in `target/release/dbdata.dll`.

2. **Replace the file:**

   Place the compiled `dbdata.dll` in the game files, replacing the real one. 

3. **Token Handling:**

   - On first run, a `token_req.txt` file will be generated with the token request and appid of the game.
   - Place your token in a file named `token.txt` in the same directory.
   - The DLL will read the token from this file allowing denuvo to work properly.

## Debug logs

- In debug builds, logs are written to a timestamped file (`dbdata_<timestamp>.log`).

## Builds

Builds are available in the [releases](https://github.com/denuvosanctuary/ubi-dbdata/releases) section of the repository. Nighly builds are also available in the [actions](https://github.com/denuvosanctuary/ubi-dbdata/actions) section.

## Disclaimer

This project is for educational and research purposes only. Use responsibly and respect software licenses.

## Thanks

Many thanks to Detanup01 for the help in the reverse engineering of the original DLL. Check out his version of the DLL [here](https://github.com/Detanup01/UplayServer)
