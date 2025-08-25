# whimsi-lib

## What does `whimsi` do?

There are 3 main sections when generating an MSI.

1. Getting user intent.
2. Translating that user intent to a useful format for MSI creation.
3. Using that translated data to create the actual MSI file.

`whimsi` handles parts 1 and 2. The 3rd part is handled by the `msi` crate.
