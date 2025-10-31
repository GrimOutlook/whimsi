# whimsi

This project is to create a command line tool that can be used on both Windows
and Linux to create an MSI deliverable. The best alternative that I can find is
the [msitools](https://gitlab.gnome.org/GNOME/msitools) project but the
documentation for that tool is nearly non-existent and many features are lacking
such as CustomActions which is what this package aims to fix.

The goal of this project is currently not to include every supported features
for MSIs. That may become it's goal in the future but for now I'm just aiming
to provide the most useful features. Issue submissions and interactions with
those issues will drive what features are added next. That and what I need at the time

If you end up using this package I'd love to know as it helps me stay motivated
to continue working on projects like these! So drop a star or shoot me a message
if you get any use out of it!


## Capabilities

- N/A (Working on it ;p)

## Development

- [Compound File Binary File Format](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-cfb/53989ce4-7b05-4f8d-829b-d08d6148375b):
Information on how to read a file and interpret the data as a CFB file.
- [OLE Property Set](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oleps/bf7aeae8-c47a-4939-9f45-700158dac3bc):
Information on how to parse the "_SummaryInformation" CFB stream into usable summary information.
- [OLE Data Structures](https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-oleds/85583d21-c1cf-4afe-a35f-d6701c5fbb6f):
Information on how to parse the CFB stream data into MSI tables and streams.
- [MSI Reference Material](https://learn.microsoft.com/en-us/windows/win32/msi/windows-installer-reference):
Information on MSI table layouts, data types, and relations.

## Ongoing Troubleshooting Notes


### Corrupted Tables

Using the current codebase the Directory, AdminUISequence, and
InstallUISequence tables all give failures when trying to view in Orca
(alongside the install failing). If I change the order of when the tables are
written I add additional tables to that list such as the Property,
AdminExecuteSequence.

When disabling the sorting of entries before insertion it can be seen that the
order of the properties in the command line example does not change when
inspected using whimsi, but the table is still unreadable by Orca and `msiexec`.


#### Theory
I have a hypothesis that what is happening is actually a storage problem. I
think that when the data is finally written to the MSI it's too large to fit in
one FAT block so it must be stored over multiple. When this happens the data is
improperly stored in some way, resulting in select tables becoming unreadable.

##### Caveats
This would require all of the tables be stored in 1 CFB stream, since all of
the tables are extremely small it would likely be the only explanation for why
a table would cross a section boundary

#### Leads

- [ ] Check if tables are stored in 1 CFB stream, or multiple.
    - If tables are stored in multiple streams
        - [ ] Dump the raw stream data from the working property example and
          the broken property example (with sorting turned off) and see if the
          data in the dump files is any different.
- [ ] Make parsing script for 010 editor for CFB/OLE/Property Set/MSI files.
    - Differences between the `sortingon.msi` and `sortingoff.msi` are minimal
      when compared in 010 editor. Need to elucidate what exact meaning the
      bytes that differentiate mean, effect, and how that information is being
      generated.
