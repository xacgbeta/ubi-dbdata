# Ubisoft Denuvo dbdata.dll

This project provides a Rust implementation of a DLL (`dbdata.dll`) that emulates the Ubisoft Denuvo token interface. It is intended for research, reverse engineering, and interoperability purposes.

## SecureDLC

SecureDLC was introduced by Ubisoft in the September 11th 2025 patch of Assassin's Creed Shadows. It requires a new type of token called `ownershipList` that is obtained through Ubisoft APIs with the normal denuvo token and a list of dlcs that the original request token was generated with. The DLL will also try to read the `ownership` field from the `token.ini` file if present.

A listof dlcs is also needed to make this ownershipList token work. The list can be specified in the `dbdata.ini` in this format:

```ini
[dlcs]
dlcs=DLC1,DLC2,DLC3
```

If this file is not present, the DLL will try to read the dlcs from the `upc_r2.ini` file.

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
   - Place your token in a file named `token.ini` in the same directory.
   - The DLL will read the token from this file allowing denuvo to work properly.

### Format of `token.ini`:
```ini
[token]
token=<your_token_here>
ownership=<optional_ownership_list_token_here>
```

## Debug logs

- In debug builds, logs are written to a timestamped file (`dbdata_<timestamp>.log`).

## Builds

Builds are available in the [releases](https://github.com/denuvosanctuary/ubi-dbdata/releases) section of the repository. Nightly builds are also available in the [actions](https://github.com/denuvosanctuary/ubi-dbdata/actions) section.

## Disclaimer

This project is for educational and research purposes only. Use responsibly and respect software licenses.

## Thanks

Many thanks to Detanup01 for the help in the reverse engineering of the original DLL. Check out his version of the DLL [here](https://github.com/Detanup01/UplayServer)
