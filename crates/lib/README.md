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

## Developer Notes

### Assumptions

The only portion of the code that is LLM generated are the long enums with many
cases so I could save time and not generating more allows me to learn about the
ins-and-outs of MSIs. I would consult with an LLM to try to get more detailed
information and then try to verify that information using official sources
(namely [Microsoft](https://learn.microsoft.com/en-us/windows/win32/msi)) but
there are instances in the code where the Microsoft documentation was unclear
and I was forced to take the LLM at it's word (with the knowledge that the
information sounds plausible/likely). I note the cases where this has occurred
below so maybe someone can correct me.

#### Sequence Number 0 for File Table Entry

In the documentation for sequence numbers in the File table it explicitly says
that sequence numbers must be "greater than or equal to 1". I became curious if
there was a use-case for making a File table entry but not including it on a
media (so a sequence number would not be needed, and may even cause undesirable
behavior) so I asked Gemini if this case was valid.

The LLMs response was that that use-case does appear when trying to duplicate a
file or when trying to use a file from external media. This makes sense as the
DuplicateFile table has a required column that contains the primary key of a
File table entry. A detractor to this point is that the DuplicateFile
documentation also states that "The original file must be a file installed by
the InstallFiles action". I'm almost certain this means the InstallFiles action
called by the currently installing MSI but it could theoretically mean it must
reference a file that was placed by that action by *any* MSI. It could also
just be wrong as I cannot find any other method of duplicating files already on
the system (which seems very useful for backing files up before operating).

I also could not find other documentation on if this is valid or if a sequence
number of 0 will cause issues so I'm going to take the LLMs word on it as it
sounds plausible and the issues can be dealt with in the future should they
arise.

### Links

- [MSI Errorcode Reference](https://learn.microsoft.com/en-us/windows/win32/msi/windows-installer-error-messages)
- [Windows Errorcode Reference](https://winprotocoldoc.z19.web.core.windows.net/MS-ERREF/%5bMS-ERREF%5d.pdf)
