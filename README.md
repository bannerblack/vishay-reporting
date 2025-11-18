entity = { path = "entity" }
migration = { path = "migration" }

1. Create user
2. Create Finished good
3. Create Report
4. Create Individual Tests in Report

# Tables

User:
- id
- name
- username
- preferences
- addedby
- createdat
- updatedat

FG:
- id
- fg
- rev
- customer

Report:
- id
- fgid (fk)
- attributes
- addedby
- createdat
- updatedat

Test:
- id
- reportid (fk)
- fgid (fk)
- testtype
- frequency
- voltage
- minimum
- maximum
- uom
- primarypins
- secondarypins
- shortedpins
- description
- addedby
- createdat
- updatedat