#!/usr/bin/env python
import os
import glob
import sys

from PIL import Image

def to_bytes(img):
    out = []
    rows, remain = divmod(img.height, 8)
    rowheights = [8] * rows
    if remain:
        rowheights.append(remain)
    for row, height in enumerate(rowheights):
        for col in range(img.width):
            val = 0
            for i in range(height):
                pixel = img.getpixel((col, (8*row) + i))
                pixel_bit = 0 if pixel < 127 else 1
                val |= pixel_bit << i
            out.append("0x%0.2x" % val)
    return out


def main():
    print "use display::Image;"
    print
    for fname in glob.glob("*.bmp"):
        base, _ = os.path.splitext(fname)
        img_name = base.upper().replace(" ", "_")
        img = Image.open(fname)
        parts = to_bytes(img)
        stride = 12
        print 
        print "pub const _%s: [u8;%s] = [" % (img_name, len(parts))
        for i in range(0, len(parts), stride):
            bit = parts[i:i+stride]
            print "  " + ", ".join(bit) + ","
        print "];"

        print "pub const %s: Image = Image { data: &_%s, width: %s, height: %s };" % (img_name, img_name, img.width, img.height)


if __name__ == "__main__":
    sys.exit(main(*sys.argv[1:]))
