# cjuggler (cj)
Config Juggler

Parameters:
```
name - string; name of the predefined configuration
sep - string; indicates how fields are separated, e.g. "\t"
fields_no - number; number of fields, including the key
key_pos - number; position of the key field
strip - true/false; whether it should strip of white spaces at the beginning and end of the line
```
Config example under ``examples/config/cjuggler.json``

Use:
```
cj -n <format_name> -a <file_alias> 
cj -n <format_name> -f <file_name>
cj -d <separator> -i <fields_no> -k <key_pos> -s <strip> -c <comment>
```
