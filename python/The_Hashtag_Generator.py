

from numpy import empty


def generate_hashtag(s):
    if (s == '') or (len(s) > 140) :
        return False
    else:
        result = []
    arr = s.split()
    for i in range(0, len(arr)):
        result.append(arr[i].capitalize())
    return '#'+(''.join(result))
    

print(generate_hashtag(''))