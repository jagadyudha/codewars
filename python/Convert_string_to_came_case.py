# Convert string to camel case

# Examples
# "the-stealth-warrior" gets converted to "theStealthWarrior"
# "The_Stealth_Warrior" gets converted to "TheStealthWarrior"

import re

def to_camel_case(text):
    text_split = re.split('-|_',text)
    result_arr = []
    for i in range (0, len(text_split)):
        if (i>0):
            result_arr.append(text_split[i].title())
        else:
            result_arr.append(text_split[i])
    return ''.join(result_arr)
    

print(to_camel_case("The_Stealth-Warrior"))