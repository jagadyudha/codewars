'''
RGB To Hex Conversion

The rgb function is incomplete. 
Complete it so that passing in RGB decimal values will result in a hexadecimal representation being returned. Valid decimal values for RGB are 0 - 255. Any values that fall out of that range must be rounded to the closest valid value.
Note: Your answer should always be 6 characters long, the shorthand with 3 will not work here.

The following are examples of expected output values:
rgb(255, 255, 255) # returns FFFFFF
rgb(255, 255, 300) # returns FFFFFF
rgb(0,0,0) # returns 000000
rgb(148, 0, 211) # returns 9400D3
'''

def rgb(r, g, b):
    r = '{:x}'.format(255) if r > 255 else '0'+'{:x}'.format(0) if r <= 0 else '0'+'{:x}'.format(r) if r <= 10 else '{:x}'.format(r)
    g = '{:x}'.format(255) if g > 255 else '0'+'{:x}'.format(0) if g <= 0 else '0'+'{:x}'.format(g) if g <= 10 else '{:x}'.format(g)
    b = '{:x}'.format(255) if b > 255 else '0'+'{:x}'.format(0) if b <= 0 else '0'+'{:x}'.format(b) if b <= 10 else '{:x}'.format(b)
    result = r+g+b
    return result.upper()

print(rgb(1,2,3))