<!-- @format -->

# Notes

Logic Gate Propagation Delay
flip flops

## Karnaugh Map

K-map can take two forms:

1. Sum of product (SOP)
2. Product of Sum (POS)

**Steps to Solve Expression using K-map**

    1. Select the K-map according to the number of variables.
    2. Identify minterms or maxterms as given in the problem.
    3. For SOP put 1’s in blocks of K-map respective to the minterms (0’s elsewhere).
    4. For POS put 0’s in blocks of K-map respective to the max terms (1’s elsewhere).
    5. Make rectangular groups containing total terms in power of two like 2,4,8 ..(except 1) and try to cover as many elements as you can in one group.
    6. From the groups made in step 5 find the product terms and sum them up for SOP form.

## Optimization

The Critical Path Optimization Principle

This hierarchy explains why critical paths (longest timing paths) are optimized by:

Replacing XORs with NANDs where possible
Balancing paths so no path has too many slow gates in series
Using larger drive strength gates for high fan-out
Pipeline insertion to break long chains

### Example: Adder Design

A naive ripple-carry adder has a critical path with:

1 XOR (for initial sum) + multiple AND/OR gates in carry chain
A carry-lookahead adder replaces this with:

More NAND/NOR gates in parallel
Fewer gates in series
Result: Faster despite more total gates

## Wire Delay

Resistance (R)

Wire resistance: R = ρ × L / (W × H)

ρ = resistivity (copper/aluminum)
L = length (gets worse quadratically!)
W = width, H = height (shrinking makes it worse)
Example: In 7nm, minimum width wires have ~1-10Ω/□
A 100μm wire might have 1000Ω resistance! 2. Capacitance (C)

Wire-to-wire capacitance (lateral)
Wire-to-substrate capacitance (vertical)
Fringe capacitance (edges of wire)
Total C ∝ L (length) 3. The RC Time Constant

τ = R × C
Since R ∝ L and C ∝ L → τ ∝ L²
Doubling wire length quadruples delay!

## On-Chip Optics
