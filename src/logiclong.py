import math

# credit: https://stackoverflow.com/questions/2063425/python-elegant-inverse-function-of-intstring-base
# convert string to any base, required to conv to
# base14 string, as format does not support base14
def digit_to_char(digit):
    if digit < 10:
        return str(digit)
    return chr(ord("a") + digit - 10)


def str_base(number, base):
    if number < 0:
        return "-" + str_base(-number, base)
    (d, m) = divmod(number, base)
    if d > 0:
        return str_base(d, base) + digit_to_char(m)
    return digit_to_char(m)


# Code generation here
def dec2rdx(num):
    arr = [x for x in format(num, "x")]
    print(arr)
    rv = ""
    for _ in range(4):
        if len(arr) == 0:
            nr = "0"
        else:
            nr = arr.pop()
        if len(arr) == 0:
            nl = "0"
        else:
            nl = arr.pop()
        nd = nl + nr
        rv = str(int(nd, 16)) + "," + rv
    return rv


def id_to_tag(lowID, highID):
    arr = ["0", "2", "8", "9", "P", "Y", "L", "Q", "G", "R", "J", "C", "U", "V"]
    frdx = str(dec2rdx(lowID) + dec2rdx(highID)).split(",")[:-1]
    if len(frdx) != 8:
        return -1
    frdx = [int(x) for x in frdx]
    low = 0
    high = 0
    for i in range(4):
        low += frdx[i]
        low *= 0x100
    low /= 0x100
    print(low)
    for i in range(4, 8):
        high += frdx[i]
        high *= 0x100
    high /= 0x100
    print(high)
    total = low + high * 0x100
    total = int(total)
    out = str_base(total, 14)
    out = [x for x in out]
    for i in range(len(out)):
        out[i] = arr[int(out[i], 16)]
    return "".join(out)


def tag_to_id(tag):
    tag = tag.upper()
    tag = tag.replace("#", "").replace("O", "0")
    arr = ["0", "2", "8", "9", "P", "Y", "L", "Q", "G", "R", "J", "C", "U", "V"]
    tag = [x for x in tag]
    total = 0
    i = 0
    while len(tag) > 0:
        # print(tag)
        ch = tag.pop()
        total += arr.index(ch) * math.pow(14, i)
        i += 1
        print(total, "total")
    total = int(total)
    print(total, "total")
    lowID = total % 256
    highID = math.floor(total / 256)
    return lowID, highID


lowID, highID = tag_to_id("#QL82UUGGG")
print(lowID, highID)
tag = id_to_tag(9, 3130736)
print("#" + tag)
