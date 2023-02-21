#!/usr/bin/env python3
import sys
import math
from decimal import *
from PIL import Image

m = 32
n = 0

class FixedPoint:
    def __init__(self, val, m_bits, n_bits, shift=True):
        self.m_bits = m_bits
        self.n_bits = n_bits
        self.total_bits = m_bits + n_bits
        self.mask = 0
        for i in range(self.total_bits):
            self.mask = (self.mask << 1) | 1

        value = int(val)
        if shift:
            value <<= n_bits;
        self.value = value & self.mask # TODO handle floats properly here

    def get_signed(self):
        sign_bit = 1 << (self.total_bits - 1)
        value = (self.value & (sign_bit - 1)) - (self.value & sign_bit)
        return value

    def __str__(self):
        return str(self.get_signed())

    def check_valid(self, rhs):
        assert self.m_bits == rhs.m_bits
        assert self.n_bits == rhs.n_bits

    def __add__(self, rhs):
        self.check_valid(rhs)
        return FixedPoint(self.value + rhs.value, self.m_bits, self.n_bits, shift=False)

    def __sub__(self, rhs):
        self.check_valid(rhs)
        return FixedPoint(self.value - rhs.value, self.m_bits, self.n_bits, shift=False)

    def __mul__(self, rhs):
        self.check_valid(rhs)
        value = (self.value * rhs.value) >> self.n_bits
        return FixedPoint(value, self.m_bits, self.n_bits, shift=False)

    def __ge__(self, rhs):
        self.check_valid(rhs)
        return self.get_signed() >= rhs.get_signed()

    def __lt__(self, rhs):
        self.check_valid(rhs)
        return self.get_signed() < rhs.get_signed()

    def __eq__(self, rhs):
        self.check_valid(rhs)
        return self.value == rhs.value

    def __neg__(self):
        neg_value = (-self.value) & self.mask
        return FixedPoint(neg_value, self.m_bits, self.n_bits, shift=False)

    # TODO fix max for negative numbers
    def max(self, rhs):
        self.check_valid(rhs)
        value = self if self.value > rhs.value else rhs
        return value


def edge(x, y, xi, yi, dXi, dYi):
    return (x - xi)*dYi - (y - yi)*dXi

def draw_pixel(x, y, im=None, v=None):
    if im is not None:
        im.putpixel((x, y), (255, 0, 0))
    else:
        print(f"Emit {x} {y}")

def draw_triangle(x0, y0, x1, y1, x2, y2, v0=0, v1=0, v2=0):
    p5 = Decimal(0.5)
    x0 += p5
    y0 += p5
    x1 += p5
    y1 += p5
    x2 += p5
    y2 += p5
    #max_y = y0.max(y1.max(y2))
    #max_x = x0.max(x1.max(x2))
    (low_x1, low_y1) = (x0, y0) if y0 < y1 else (x1, y1)
    (low_x, low_y) = (low_x1, low_y1) if low_y1 < y2 else (x2, y2)
    (high_x1, high_y1) = (x0, y0) if y0 > y1 else (x1, y1)
    (high_x, high_y) = (high_x1, high_y1) if high_y1 > y2 else (x2, y2) 
    dX0 = x0 - x2
    dX1 = x1 - x0
    dX2 = x2 - x1
    dY0 = y0 - y2
    dY1 = y1 - y0
    dY2 = y2 - y1
    start_x = Decimal(0.5)
    start_y = Decimal(0.5)
    e0 = edge(start_x, start_y, x0, y0, dX0, dY0)
    e1 = edge(start_x, start_y, x1, y1, dX1, dY1)
    e2 = edge(start_x, start_y, x2, y2, dX2, dY2)
    #area = e0 + e1 + e2
    #dYV = (dY0*v0 + dY1*v1 + dY2*v2) / area
    #dXV = (dX0*v0 + dX1*v1 + dX2*v2) / area

    #v = (e0*v0 + e1*v1 + e2*v2) / area

    print(f"Edge values are")
    print(f"\te0={e0}, e1={e1}, e2={e2}")
    print(f"\tdX0={dX0}, dX1={dX1}, dX2={dX2}")
    print(f"\tdY0={dY0}, dY1={dY1}, dY2={dY2}")

    dX0 = FixedPoint(dX0, m, n)
    dY0 = FixedPoint(dY0, m, n)
    dX1 = FixedPoint(dX1, m, n)
    dY1 = FixedPoint(dY1, m, n)
    dX2 = FixedPoint(dX2, m, n)
    dY2 = FixedPoint(dY2, m, n)
    e0 = FixedPoint(e0, m, n)
    e1 = FixedPoint(e1, m, n)
    e2 = FixedPoint(e2, m, n)
    zero = FixedPoint(0, m, n)

    starting_e0 = e0
    starting_e1 = e1
    starting_e2 = e2

    im = Image.new(mode="RGB", size=(256,144))

    for y in range(144):
        for x in range(256):
            if e0 >= zero and e1 >= zero and e2 >= zero:
                draw_pixel(x, y, im)

            e0 = e0 + dY0
            e1 = e1 + dY1
            e2 = e2 + dY2

        e0 = starting_e0 - dX0
        e1 = starting_e1 - dX1
        e2 = starting_e2 - dX2

        starting_e0 = e0
        starting_e1 = e1
        starting_e2 = e2

    im.show()

def main():
    if len(sys.argv) < 7:
        print(f"Call like '{sys.argv[0]} x0 y0 x1 y1 x2 y2'")
        return 0

    x0 = Decimal(int(sys.argv[1])) 
    y0 = Decimal(int(sys.argv[2])) 
    x1 = Decimal(int(sys.argv[3])) 
    y1 = Decimal(int(sys.argv[4])) 
    x2 = Decimal(int(sys.argv[5])) 
    y2 = Decimal(int(sys.argv[6])) 

    draw_triangle(x0, y0, x1, y1, x2, y2)

    return 0

if __name__ == "__main__":
    sys.exit(main())
