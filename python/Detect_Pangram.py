
'''
Detect Pangram

A pangram is a sentence that contains every single letter of the alphabet at least once.
For example, the sentence "The quick brown fox jumps over the lazy dog" is a pangram, because it uses the letters A-Z at least once (case is irrelevant). 
Given a string, detect whether or not it is a pangram. 
Return True if it is, False if not. 
Ignore numbers and punctuation.'''

def is_pangram(s):
    result = []
    current = 0
    s = s.lower()
    s = list(s)
    s.sort()
    alphabet = 'abcdefghijklmnopqrstuvwxyz'
    for i in s:
        if i in list(alphabet) and (i != current):
           result.append(i)
           current = i

    if len(result) == 26:
        return True
    else:
        return False

print(is_pangram("The quick brown fox jumps over the lazy dog"))