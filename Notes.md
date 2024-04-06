## Overlays
Overlays cause a lot of issues for data representation. Overlays are structured such that there is the base datapack, and then partial datapacks stored as overlays which are applied on top of the base datapack, overwriting any files which appear in both.

The issues are the following:
- Overlays contain entirely separate copies of files, even for small edits or format changes
- In most cases, the data between these should probably be linked, but there are cases where it may be desirable to have different values (even for shared data)
- Data which differs per format may have multiple formats which use the same data (e.g. there are three formats for value X, and formats 1 and 2 are identical for that value type, while 3 differs)

Proposed Solution:
- Main datapack configuration has a checklist for supported versions
  - This has the additional complication that older versions do not support overlays, but this should be transparent to the user except on export
- Store a copy of each file per supported format
    - Data should by default be stored the same per included format
    - An option should exist to maintain different values per format, even if the actual data representation did not change (this allows for things like changing blocks based on MC version)
    - Data which did change format should be required to be filled out for each version
      - Data with only names changed should be automatically synced, with the option for decoupling
      - Data which represents the same idea should be linked together, but require data input for each unique representation
      - Data which is absent in other formats should be clearly communicated as such
- Type definitions for data representation
  - Current setup is Elements for project manipulation, storing serializable data inside it - serializable data is per-format
  - Elements should expose data values through public api visible to rest of project
    - Data values store format definitions and interact with internal serializable data
    - Synced vs manual state stored here as well