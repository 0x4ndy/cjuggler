# cjuggler (cj)
Config Juggler

Parameters:

name - string; name of the predefined configuration
sep - string; indicates how fields are separated, e.g. "\t"
fields-no - number; number of fields, including the key
key-pos - number; position of the key field
strip - true/false; whether it should strip of white spaces at the beginning and end of the line

Config example under examples/config/cjuggler.json

Use:

cj -n \<format-name\> -a \<file-alias\> 

cj -n \<format-name\> -f \<file-name\>

cj -d \<separator\> -i \<fields-no\> -k \<key-pos\> -s \<strip\> -c \<comment\>
