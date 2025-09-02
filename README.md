# whimsi-lib

## What does `whimsi` do?

There are 3 main sections when generating an MSI.

1. Getting user intent.
2. Translating that user intent to a useful format for MSI creation.
3. Using that translated data to create the actual MSI file.

`whimsi-lib` provides an easy API for part 1 and handles part 2 once all
modifications by the user have been made. The 3rd part is handled by the `msi`
crate.


## Testing Areas

1. User intent is translated into intermediary representation correctly.
2. Intermediary representation is translated into `msi` crate table information correctly.
3. User intent is translated into final output MSI.
